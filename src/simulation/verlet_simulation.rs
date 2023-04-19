use bevy::prelude::*;
use crate::{ physics, resources, spacecraft };
use bevy::math::DVec3;
use std::ops::{ Mul };
use uom::si::length::meter;
use uom::si::mass::kilogram;

use physics::force_vector::ForceVector as ForceVector;
use physics::position_vector::PositionVector as PositionVector;

pub fn verlet_simulation(
    time:                   Res<Time>, 
    esail_query:            Query<&spacecraft::esail::ESail>,  
    solar_wind_parameters:  Res<resources::SolarWindParameters>,
    craft_params:  Res<spacecraft::SpacecraftParameters>,
    mut verlet_query:       Query<&mut physics::verlet_object::VerletObject>,
    mut sim_params:         ResMut<resources::SimulationParameters>,
    ) {

    let esail = esail_query.single();

    // Timesteps since last frame
    let timesteps = timestep_calculation(&time, &mut sim_params);

    for _ in 0..timesteps { 

        // VERLET INTEGRATION: Forces are calculated for every element

        for entity in esail.deployed_elements.iter() {  // Iterating over esail DEPLOYED elements, in order.

            let mut verlet_object = verlet_query.get_mut(*entity).expect("No sail element found");

            verlet_integration(&mut sim_params, &mut verlet_object, &craft_params, &solar_wind_parameters);
        }

        // CONSTRAINT LOOP

        for _ in 0..sim_params.iterations {

            //for index in 1..esail.elements.len() {  // Skipping first item
            for index in 0..esail.elements.len() {  // Why are these two the same!?

                // Relative position between element and preceding element, as a PositionVector
                let relative_position_between_elements = esail.vector_to_previous_element(index, &verlet_query);
                let distance_between_elements = relative_position_between_elements.clone().length().get::<meter>();

                // Desired distance between elements (in meters)
                let desired_relative_position_between_elements = craft_params.segment_length();

                let difference = if distance_between_elements > 0.0 {
                    (desired_relative_position_between_elements.get::<meter>() - distance_between_elements) / distance_between_elements
                } else {
                    0.0
                };

                let correction_vector = relative_position_between_elements.mul(0.5 * difference);

                // UPDATING POSITIONS
                
                let mut current_verlet_object = verlet_query.get_mut(esail.elements[index]).expect("No previous sail element found");

                current_verlet_object.correct_current_coordinates(correction_vector.clone());

                // Changing previous element if previous element is not the first.
                //if index > 1 {
                if index > 0 {
                    let mut preceding_verlet_object = verlet_query.get_mut(esail.elements[index - 1]).expect("No previous sail element found");
                    //preceding_verlet_object.correct_current_coordinates(correction_vector.mul(-1.0));
                    if preceding_verlet_object.is_deployed {
                        // Maybe a method to give the negative?
                        preceding_verlet_object.correct_current_coordinates(correction_vector.mul(-1.0));
                    }
                }
            }
        }
    }
}

/// Updates the position of a verlet object
fn verlet_integration(
    sim_params:  &mut ResMut<resources::SimulationParameters>,
    verlet_object:          &mut physics::verlet_object::VerletObject,
    craft_params:  &Res<spacecraft::SpacecraftParameters>,
    solar_wind:             &Res<resources::SolarWindParameters>,
    ){

    // Forces per verlet (so, per segment)

    //// Centrifugal force (Along x for now, this needs to change)

    let centrifugal_force_magnitude = craft_params.segment_mass() * verlet_object.current_coordinates.clone().length() 
        * craft_params.angular_velocity() * craft_params.angular_velocity();

    let centrifugal_force_direction = DVec3::new(1.0, 0.0, 0.0);

    let centrifugal_force = ForceVector::from_direction(centrifugal_force_magnitude, centrifugal_force_direction);

    //// Coulomb drag force
    
    let coulomb_force_magnitude= coulomb_force_per_meter(&solar_wind, &craft_params) * craft_params.segment_length();

    let coulomb_force = ForceVector::from_direction(coulomb_force_magnitude, solar_wind.direction); 

    //// Total force

    let total_force = coulomb_force + centrifugal_force;    // This is a ForceVector containing uom quantities

    let total_acc = total_force.clone() / craft_params.segment_mass().get::<kilogram>();    // What quantity is this? It's force!!

    let x_acc = total_force.x() / craft_params.segment_mass();
    let y_acc = total_force.y() / craft_params.segment_mass();
    let z_acc = total_force.z() / craft_params.segment_mass();

    // Not making an acceleration uom vector just for this if the result is a position vector anyways
    // Need a better name for this
    let x_acceleration = total_force.x() / craft_params.segment_mass() * sim_params.timestep_s * sim_params.timestep_s;
    let y_acceleration = total_force.y() / craft_params.segment_mass() * sim_params.timestep_s * sim_params.timestep_s;
    let z_acceleration = total_force.z() / craft_params.segment_mass() * sim_params.timestep_s * sim_params.timestep_s;

    // Wondering if these units are correct
    let position_from_acceleration = PositionVector::new(x_acceleration, y_acceleration, z_acceleration);

    // Next position calculation (formula from here: https://www.algorithm-archive.org/contents/verlet_integration/verlet_integration.html)
    let next_coordinates = verlet_object.current_coordinates.clone().mul(2.0) - verlet_object.previous_coordinates.clone() + position_from_acceleration;

    // Damping here, before the update of coordinates?

    
    // Updating verlet coordinates
    verlet_object.update_coordinates(next_coordinates);

}

/// Calculates how many timesteps should happen in the current frame, considering any potential unspent time from the previous frame.
fn timestep_calculation(
    time: &Res<Time>,
    sim_params: &mut ResMut<resources::SimulationParameters>,
    ) -> i32 {

    let elapsed_time = time.delta_seconds() as f64 + sim_params.leftover_time; // Elapsed time + leftover time from previous frame

    let timesteps = (elapsed_time / sim_params.timestep).floor() as i32; // Number of timesteps for the current frame

    let leftover_time = elapsed_time - timesteps as f64 * sim_params.timestep;  // Leftover time saved for next frame
    sim_params.leftover_time = leftover_time;

    return timesteps;
}

// From janhunen2007, equation 8. Corroborate all the results. And recheck the equations too.
// Should this go inside the physics folder, in its own file?
#[allow(non_snake_case)]
pub fn coulomb_force_per_meter( 
    solar_wind:         &Res<resources::SolarWindParameters>, 
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
