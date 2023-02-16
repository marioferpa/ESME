// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use std::f64::consts;
use bevy::prelude::*;
use crate::{ components, resources };

use uom::si::*;

pub struct PhysicsPlugin;   // Plugins are structs, therefore they can hold data!

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_esail_voltage)         // "Charges" the sail with up to the chosen potential
            .add_system(Self::verlet_simulation)            // Calculates new positions
            .add_system(Self::update_transform_verlets)     // Updates the position of the graphics
            .add_system(Self::update_center_of_mass)        // Updates position of the center of mass
            ;
    }
}

impl PhysicsPlugin {

    /// Updates the potential of every conductor to whatever the gui is showing
    fn update_esail_voltage(
        spacecraft_parameters:  Res<resources::SpacecraftParameters>,
        mut electrical_query:   Query<&mut components::ElectricallyCharged>,
        ) {

        for mut electrical_element in electrical_query.iter_mut() {
            electrical_element.potential = spacecraft_parameters.wire_potential;
        }
    }

    /// Simulation proper
    fn verlet_simulation(
        time:                   Res<Time>, 
        esail_query:            Query<&components::ESail>,
        mut simulation_parameters:         ResMut<resources::SimulationParameters>,
        spacecraft_parameters:  Res<resources::SpacecraftParameters>,
        solar_wind_parameters:  Res<resources::SolarWindParameters>,
        mut verlet_query:       Query<(&mut components::VerletObject, &components::Mass), With<components::SailElement>>,
        ) {

        let esail = esail_query.single();

        let timesteps = timestep_calculation(&time, &mut simulation_parameters);

        if simulation_parameters.debug { println!("New frame ------------------"); }

        for _ in 0..timesteps { 

            if simulation_parameters.debug { println!("New timestep ---------------"); }

            // VERLET INTEGRATION
            // Forces are calculated and applied for each esail element

            for element in esail.elements.iter() {  // Iterating over esail elements, in order.

                let (mut verlet_object, _) = verlet_query.get_mut(*element).expect("No sail element found");

                if verlet_object.is_deployed {
                    verlet_integration(&mut simulation_parameters, &mut verlet_object, &spacecraft_parameters, &solar_wind_parameters);
                }
            }

            // CONSTRAINT LOOP
            // Final position is corrected taking into account the constraints of the system.
            // All operations in pixels, I believe.

            for _ in 0..simulation_parameters.iterations {

                if simulation_parameters.debug { println!("New constraint iteration ---"); }

                for (index, sail_element) in esail.elements.iter().enumerate().skip(1) {    // Iterating over the sail elements in order. Skips the first.
                                                                                            // Needed if I'm already checking for deployment?

                    // Information from current element
                    let (current_verlet_object, _) = verlet_query.get(*sail_element).expect("No previous sail element found");

                    let current_element_x = current_verlet_object.current_x;
                    let current_element_y = current_verlet_object.current_y;

                    // Information from previous element
                    let prev_sail_element = esail.elements[index - 1];
                    let (prev_verlet_object, _) = verlet_query.get(prev_sail_element).expect("No previous sail element found");

                    let prev_element_x = prev_verlet_object.current_x;
                    let prev_element_y = prev_verlet_object.current_y;

                    // Calculating distance between current sail element and previous element in the line (in pixels, right?)
                    let diff_x = current_element_x - prev_element_x;
                    let diff_y = current_element_y - prev_element_y;
                    let pixels_between_elements = (diff_x * diff_x + diff_y * diff_y).sqrt();

                    if simulation_parameters.debug {
                        println!("Index: {} | Distance between elements (px): {}", index, pixels_between_elements);
                    }

                    let mut difference = 0.0;

                    let desired_pixels_between_elements = spacecraft_parameters.segment_length().value * simulation_parameters.pixels_per_meter as f64;

                    if pixels_between_elements > 0.0 {
                        difference = (desired_pixels_between_elements - pixels_between_elements) / pixels_between_elements;
                    }

                    // This shouldn't be .5 if one object is not deployed, although I believe it tends to the correct spot anyways.
                    let correction_x = diff_x * 0.5 * difference;
                    let correction_y = diff_y * 0.5 * difference;

                    // UPDATING POSITIONS
                    // Yes, I'm querying both again, can't find a cleaner way to do it.
                    
                    let (mut current_verlet_object, _) = verlet_query.get_mut(*sail_element).expect("No previous sail element found");
                    
                    if current_verlet_object.is_deployed {
                        current_verlet_object.current_x += correction_x;
                        current_verlet_object.current_y += correction_y;
                    }

                    let (mut prev_verlet_object, _) = verlet_query.get_mut(prev_sail_element).expect("No previous sail element found");
                    
                    if prev_verlet_object.is_deployed {
                        prev_verlet_object.current_x -= correction_x;
                        prev_verlet_object.current_y -= correction_y;
                    }
                }
            }
        }
    }

    /// Updates the transform of the verlet objects after the simulation, so that the graphics get updated.
    fn update_transform_verlets(
        mut verlet_query: Query<(&components::VerletObject, &mut Transform)>,
        ){
        
        for (verlet_object, mut transform) in verlet_query.iter_mut() {
            transform.translation.x = verlet_object.current_x as f32;
            transform.translation.y = verlet_object.current_y as f32;
        }

        // Should this update the rotation of the segments too?
    } 

    /// Updates position and visibility of the center of mass
    fn update_center_of_mass(
        simulation_parameters:     Res<resources::SimulationParameters>,
        mass_query:     Query<(&Transform, &components::Mass), Without<components::CenterOfMass>>,
        mut com_query:  Query<(&mut Transform, &mut Visibility), With<components::CenterOfMass>>, 
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

        let (mut com_transform, mut com_visibility) = com_query.single_mut();

        com_transform.translation.x = center_mass_x;
        com_transform.translation.y = center_mass_y;

        // Should this go in graphics? I believe so.
        com_visibility.is_visible = simulation_parameters.com_visibility;
    }
}


/// Updates the position of a verlet object
fn verlet_integration(
    simulation_parameters:  &mut ResMut<resources::SimulationParameters>,
    verlet_object:          &mut components::VerletObject,
    spacecraft_parameters:  &Res<resources::SpacecraftParameters>,
    solar_wind_parameters:  &Res<resources::SolarWindParameters>,
    ){

    // CALCULATION OF VELOCITIES

    let current_position_x  = verlet_object.current_x;
    let current_position_y  = verlet_object.current_y;

    let previous_position_x = verlet_object.previous_x;
    let previous_position_y = verlet_object.previous_y;

    // Maybe I shouldn't call these velocities, even if they are proportional to that.
    let velocity_x = current_position_x - previous_position_x;
    let velocity_y /* wait what */ = current_position_y - previous_position_y;


    // FORCES

    // X AXIS: Centrifugal force

    let distance_to_center = (current_position_x * current_position_x + current_position_y * current_position_y).sqrt();

    let angular_velocity = spacecraft_parameters.rpm * consts::PI / 30.0;   // What was this 30.0 for?

    let acceleration_x = distance_to_center * angular_velocity * angular_velocity;

    let next_position_x = current_position_x + velocity_x + acceleration_x.value * simulation_parameters.timestep * simulation_parameters.timestep;

    // Y AXIS: Coulomb drag
    
    //let force_per_segment = coulomb_force_per_segment(&solar_wind_parameters, &spacecraft_parameters);
    let force_per_segment = coulomb_force_per_meter(&solar_wind_parameters, &spacecraft_parameters) * spacecraft_parameters.segment_length();

    let acceleration_y = force_per_segment / spacecraft_parameters.segment_mass();

    println!("{}: {:?}", "Force per segment", force_per_segment);    
    println!("{}: {:?}", "Total force", force_per_segment * spacecraft_parameters.wire_resolution.value * spacecraft_parameters.wire_length.value);
    println!("-------------------------");

    let next_position_y = current_position_y + velocity_y + acceleration_y.value * simulation_parameters.timestep * simulation_parameters.timestep;
    //println!("{}", next_position_y);
    
    // Starting to think that the bending moment should go here too.

    // UPDATING OBJECT POSITION

    // Previous position is forgotten,
    
    // current position becomes previous position,
    verlet_object.previous_x = current_position_x;
    verlet_object.previous_y = current_position_y;

    // and next position becomes current position.
    verlet_object.current_x = next_position_x;
    verlet_object.current_y = next_position_y;
}

// From janhunen2007, equation 8. Corroborate all the results. And recheck the equations too.
#[allow(non_snake_case)]
fn coulomb_force_per_meter( 
    solar_wind:         &Res<resources::SolarWindParameters>, 
    spacecraft:         &Res<resources::SpacecraftParameters>,
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

    println!("{}: {:?}", "Force per meter", force_per_unit_length); 

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
