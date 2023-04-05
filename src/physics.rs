// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use bevy::math::DVec3;
use crate::{ components, resources, elements };

use uom::si::*;

use std::ops::{ Mul };

pub mod position_vector;
pub mod force_vector;
pub mod verlet_object;

// All operations in this plugin should be done in physical units. Get rid of pixels in verlets.
// Graphics.rs should then translate distances to pixels when needed.

pub struct PhysicsPlugin;   // Plugins are structs, therefore they can hold data!

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_system(Self::update_esail_voltage)         // "Charges" the sail with up to the chosen potential
            .add_system(Self::verlet_simulation)            // Calculates new positions
            .add_system(Self::update_center_of_mass)        // Updates position of the center of mass
            ;
    }
}

impl PhysicsPlugin {

    ///// Updates the potential of every conductor to whatever the gui is showing
    //fn update_esail_voltage(
    //    spacecraft_parameters:  Res<elements::SpacecraftParameters>,
    //    mut electrical_query:   Query<&mut components::ElectricallyCharged>,
    //    ) {

    //    for mut electrical_element in electrical_query.iter_mut() {
    //        electrical_element.potential = spacecraft_parameters.wire_potential;
    //    }
    //}

    /// Simulation proper
    fn verlet_simulation(
        time:                       Res<Time>, 
        esail_query:                Query<&elements::esail::ESail>,  // Should I add information about the pivot to ESail?
        solar_wind_parameters:      Res<resources::SolarWindParameters>,
        spacecraft_parameters:      Res<elements::SpacecraftParameters>,
        mut verlet_query:           Query<&mut verlet_object::VerletObject>,
        mut simulation_parameters:  ResMut<resources::SimulationParameters>,
        ) {

        let esail = esail_query.single();

        let timesteps = timestep_calculation(&time, &mut simulation_parameters);

        for _ in 0..timesteps { 

            // VERLET INTEGRATION: Forces are calculated and applied for each esail element

            for element in esail.elements.iter() {  // Iterating over esail elements, in order.

                let mut verlet_object = verlet_query.get_mut(*element).expect("No sail element found");

                verlet_integration(&mut simulation_parameters, &mut verlet_object, &spacecraft_parameters, &solar_wind_parameters);
            }

            // CONSTRAINT LOOP. All operations in pixels, I'm pretty sure.

            for _ in 0..simulation_parameters.iterations {

                for index in 0..esail.elements.len() {  

                    // Distance between element and preceding element (in METERS). 
                    let distance_between_elements = esail.distance_between_elements(index, &verlet_query);    // Now is a PositionVector

                    // Desired distance between elements (in METERS TOO)
                    let desired_distance_between_elements = spacecraft_parameters.segment_length();

                    // If difference is zero then I can skip all the rest, right? Perfect spot for an early return.

                    let difference = if distance_between_elements.clone().length().value > 0.0 {
                        (desired_distance_between_elements.value - distance_between_elements.clone().length().value) / distance_between_elements.clone().length().value
                    } else {
                        0.0
                    };

                    let correction_vector = if index > 0 {
                        distance_between_elements.mul(0.5 * difference)
                    } else {
                        distance_between_elements.mul(difference)
                    };

                    // UPDATING POSITIONS
                    
                    let mut current_verlet_object = verlet_query.get_mut(esail.elements[index]).expect("No previous sail element found");

                    current_verlet_object.correct_current_coordinates(correction_vector.clone());

                    // Also change previous to preceding wherever needed

                    if index > 0 {
                        //let preceding_sail_element   = esail.elements[index - 1];
                        let mut preceding_verlet_object = verlet_query.get_mut(esail.elements[index - 1]).expect("No previous sail element found");
                        preceding_verlet_object.correct_current_coordinates(correction_vector.mul(-1.0));
                    }
                }
            }
        }
    }


    /// Updates position and visibility of the center of mass
    /// Maybe this should calculate its position, and graphics.rs should update the transform
    fn update_center_of_mass(
        simulation_parameters:     Res<resources::SimulationParameters>,
        mass_query:     Query<(&Transform, &components::Mass), Without<elements::center_mass::CenterOfMass>>,
        mut com_query:  Query<&mut Transform, With<elements::center_mass::CenterOfMass>>, 
        ){

        let mut total_mass:     f32 = 0.0;  // In this particular case I don't think I should use physical units.
                                            // Transform will be in pixels, and mass units are cancelled out.
        let mut center_mass_x:  f32 = 0.0;
        let mut center_mass_y:  f32 = 0.0;

        for (transform, object_mass) in mass_query.iter() {
            total_mass    += object_mass.0.value as f32; 
            center_mass_x += transform.translation.x * object_mass.0.value as f32;
            center_mass_y += transform.translation.y * object_mass.0.value as f32;
        }

        if simulation_parameters.debug {
            println!("Total mass: {} | Center of mass: ({},{})", total_mass, center_mass_x, center_mass_y);
        }

        //let (mut com_transform, mut com_visibility) = com_query.single_mut();
        let mut com_transform = com_query.single_mut();

        com_transform.translation.x = center_mass_x;
        com_transform.translation.y = center_mass_y;

    }
}


/// Updates the position of a verlet object
fn verlet_integration(
    simulation_parameters:  &mut ResMut<resources::SimulationParameters>,
    verlet_object:          &mut verlet_object::VerletObject,
    spacecraft_parameters:  &Res<elements::SpacecraftParameters>,
    solar_wind:             &Res<resources::SolarWindParameters>,
    ){

    // Forces per verlet (so, per segment)

    //// Centrifugal force (Along x for now, this needs to change)

    let centrifugal_force_magnitude = spacecraft_parameters.segment_mass() * verlet_object.current_coordinates.clone().length()
                                        * spacecraft_parameters.angular_velocity() * spacecraft_parameters.angular_velocity();

    let centrifugal_force_direction = DVec3::new(1.0, 0.0, 0.0);

    let centrifugal_force = force_vector::ForceVector::from_direction(centrifugal_force_magnitude, centrifugal_force_direction);

    //// Coulomb drag force
    
    let coulomb_force_magnitude= coulomb_force_per_meter(&solar_wind, &spacecraft_parameters) * spacecraft_parameters.segment_length();

    let coulomb_force = force_vector::ForceVector::from_direction(coulomb_force_magnitude, solar_wind.direction); 


    //// Total force

    let total_force = coulomb_force + centrifugal_force;    // This is a ForceVector containing uom quantities

    // Not making an acceleration uom vector just for this if the result is a position vector anyways
    let x_acceleration = total_force.x() / spacecraft_parameters.segment_mass() * simulation_parameters.timestep_s * simulation_parameters.timestep_s;
    let y_acceleration = total_force.y() / spacecraft_parameters.segment_mass() * simulation_parameters.timestep_s * simulation_parameters.timestep_s;
    let z_acceleration = total_force.z() / spacecraft_parameters.segment_mass() * simulation_parameters.timestep_s * simulation_parameters.timestep_s;

    // Wondering if these units are correct
    let position_from_acceleration = position_vector::PositionVector::new(x_acceleration, y_acceleration, z_acceleration);

    // Next position calculation (formula from here: https://www.algorithm-archive.org/contents/verlet_integration/verlet_integration.html)
    let next_coordinates = verlet_object.current_coordinates.clone().mul(2.0) - verlet_object.previous_coordinates.clone() + position_from_acceleration;

    
    // Updating verlet coordinates
    verlet_object.update_coordinates(next_coordinates);

    //println!("{}: {:?}", "Force per segment", force_per_segment);    
    //// And force per meter?
    //println!("{}: {:?}", "Total force", force_per_segment * spacecraft_parameters.wire_resolution.value * spacecraft_parameters.wire_length.value);
    //println!("-------------------------");

}

// From janhunen2007, equation 8. Corroborate all the results. And recheck the equations too.
// Should this go inside the physics folder, in its own file?
#[allow(non_snake_case)]
pub fn coulomb_force_per_meter( 
    solar_wind:         &Res<resources::SolarWindParameters>, 
    spacecraft:         &Res<elements::SpacecraftParameters>,
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

/// Calculates how many timesteps should happen in the current frame, considering any potential unspent time from the previous frame.
fn timestep_calculation(
    time: &Res<Time>,
    simulation_parameters: &mut ResMut<resources::SimulationParameters>,
    ) -> i32 {

    let elapsed_time = time.delta_seconds() as f64 + simulation_parameters.leftover_time; // Elapsed time + leftover time from previous frame

    let timesteps = (elapsed_time / simulation_parameters.timestep).floor() as i32; // Number of timesteps for the current frame

    let leftover_time = elapsed_time - timesteps as f64 * simulation_parameters.timestep;  // Leftover time saved for next frame
    simulation_parameters.leftover_time = leftover_time;

    return timesteps;
}
