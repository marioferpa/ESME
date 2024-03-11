use bevy::prelude::*;
use bevy::math::DVec3;

use crate::{ physics, resources, solar_wind, spacecraft, time };

use std::ops::{ Mul };

use uom::si::length::meter;

use physics::force_vector::ForceVector as ForceVector;
use physics::position_vector::PositionVector as PositionVector;
use physics::acceleration_vector::AccelerationVector as AccelerationVector;

pub fn verlet_simulation (
    time:               Res<Time>, 
    mut esail_query:    Query<&mut spacecraft::esail::ESail>,
    solar_wind:         Res<solar_wind::SolarWind>,
    craft_params:       Res<spacecraft::SpacecraftParameters>,
    mut sim_params:     ResMut<resources::SimulationParameters>,
) {

    let mut esail = esail_query.single_mut();

    for _ in 0..time::timestep_calculation(&time, &mut sim_params) {


        // Verlet integration --------------------------------------------------

        for verlet_object in esail.elements.iter_mut() {

            new_verlet_integration(
                &mut sim_params, verlet_object, &craft_params, &solar_wind
            );
        }




        // Constraints ---------------------------------------------------------


        let desired_distance_between_elements = craft_params.segment_length();


        for _ in 0..sim_params.iterations {

            for index in 0..esail.elements.len() {



                // Vector between current and previous elements ----------------

                let current_element_coordinates = 
                    esail.elements[index].current_coordinates.clone();

                let preceding_element_coordinates = if index == 0 {
                    &esail.origin
                } else {
                    &esail.elements[index-1]
                          .current_coordinates
                };

                
                let vector_between_elements = 
                    PositionVector::from_a_to_b(
                        current_element_coordinates,
                        preceding_element_coordinates.clone(),
                    );

                let distance_between_elements = vector_between_elements.clone().length();




                // Difference to ideal -----------------------------------------


                let difference = if distance_between_elements.get::<meter>() > 0.0 {
                    (desired_distance_between_elements.get::<meter>() 
                    - distance_between_elements.get::<meter>())
                    / distance_between_elements.get::<meter>()
                } else {
                    0.0
                };



                // Correction vector -------------------------------------------

                let correction_vector = 
                    vector_between_elements.mul(0.5 * difference);

                    // Maybe a method to give the negative?
                esail.elements[index].correct_current_coordinates(correction_vector.clone().mul(-1.0));


                if esail.elements[index].is_deployed {

                    let preceding_verlet = &mut esail.elements[index - 1];

                    if preceding_verlet.is_deployed {

                        preceding_verlet.correct_current_coordinates(correction_vector);
                    }
                }
            }
        }
    }
}





// Should be moved to a submodule, and maybe change its name? It's updating the
// coordinates, maybe something like that

// It's identical to the old integration, so why does it fail?
/// Updates the position of a verlet object
fn new_verlet_integration (
    sim_params:     &mut ResMut<resources::SimulationParameters>,
    verlet_object:  &mut physics::verlet_object::VerletObject,
    craft_params:   &Res<spacecraft::SpacecraftParameters>,
    solar_wind:     &Res<solar_wind::SolarWind>,
) {

    if verlet_object.is_deployed == false { return };

    // Centrifugal force -------------------------------------------------------

    let centrifugal_force_magnitude = 
        craft_params.segment_mass() * 
        verlet_object.current_coordinates.clone().length() *
        craft_params.angular_velocity() *
        craft_params.angular_velocity();

    let centrifugal_force_direction = DVec3::new(1.0, 0.0, 0.0);

    let centrifugal_force = 
        ForceVector::from_direction(
            centrifugal_force_magnitude, 
            centrifugal_force_direction
        );




    // Coulomb drag force ------------------------------------------------------
    
    let coulomb_force_magnitude = 
        coulomb_force_per_meter(&solar_wind, &craft_params) * 
        craft_params.segment_length();

    let coulomb_force = 
        ForceVector::from_direction(
            coulomb_force_magnitude, 
            solar_wind.direction
        ); 


    // Stiffness reaction force here?
    // On the one hand it makes sense to put it here, on the other I feel that
    // the idea of verlet integration is to calculate the movement of the free
    // particle and then add the constraint. If I modeled the perpendicular
    // stiffness as a force, why not modelling the longitudinal as a force too,
    // and the verlet would stop making sense
    



    // Total force -------------------------------------------------------------

    let total_force = coulomb_force + centrifugal_force;

    verlet_object.current_force = total_force.clone();

    let acc_vector = 
        AccelerationVector::from_force(
            total_force.clone(), 
            craft_params.segment_mass()
        );

    let delta_from_acc = 
        PositionVector::from_acceleration(
            acc_vector, 
            sim_params.timestep
        );
 

    // Next position calculation 
    // formula from https://www.algorithm-archive.org/contents/verlet_integration/verlet_integration.html
    let next_coordinates = 
        verlet_object.current_coordinates.clone().mul(2.0) - 
        verlet_object.previous_coordinates.clone() 
        + delta_from_acc;

    
    // Updating verlet coordinates
    verlet_object.update_coordinates(next_coordinates);
}



// From janhunen2007, equation 8. Corroborate all the results. And recheck the equations too.
// Should this go inside the physics folder, in its own file?
// Move to physics?
#[allow(non_snake_case)]
pub fn coulomb_force_per_meter( 
    solar_wind:         &Res<solar_wind::SolarWind>, 
    spacecraft:         &Res<spacecraft::SpacecraftParameters>,
) -> uom::si::f64::RadiantExposure {    // Radiant exposure is [mass][time]⁻²

    // First: r_0, distance at which the potential vanishes
    let r0_numerator    = resources::EPSILON_0 * solar_wind.T_e;
    let r0_denominator  = solar_wind.n_0 * resources::Q_E * resources::Q_E; 
    let r_0             = 2.0 * (r0_numerator / r0_denominator).sqrt();    

    // Second: r_s, stopping distance of protons
    let exp_numerator   = resources::M_PROTON * solar_wind.velocity * solar_wind.velocity * (r_0 / spacecraft.wire_radius).ln();
    let exp_denominator = resources::Q_E * spacecraft.wire_potential; 
    let exp             = (exp_numerator / exp_denominator).exp();  
    let rs_denominator  = (exp.value - 1.0).sqrt();
    let r_s             = r_0 / rs_denominator;

    // Third: force per unit length
    let K = 3.09;   // Empirical, from Monte Carlo sims, I need to calculate this myself somehow.

    let force_per_unit_length = r_s * K * resources::M_PROTON * solar_wind.n_0 * solar_wind.velocity * solar_wind.velocity;

    //println!("{}: {:?}", "Force per meter", force_per_unit_length); 

    return force_per_unit_length;
}
