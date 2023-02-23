// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use std::f64::consts;
use bevy::prelude::*;
use std::ops::Mul;  // For multiplying DVec3
use crate::{ components, resources };

use uom::si::*;

pub struct PhysicsPlugin;   // Plugins are structs, therefore they can hold data!

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_esail_voltage)         // "Charges" the sail with up to the chosen potential
            .add_system(Self::verlet_simulation)            // Calculates new positions
            .add_system(Self::update_center_of_mass)        // Updates position of the center of mass
            //.add_system(Self::update_body_rotation)
            .add_system(Self::update_sail_rotation)
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

    fn update_body_rotation (
        time:                   Res<Time>, 
        spacecraft_parameters:  Res<resources::SpacecraftParameters>, 
        mut satellite_query:    Query<&mut Transform, (With<components::SatelliteBody>, Without<components::ESail>)>,
        ){

        // Sat should have an angle variable (in uom units!) that I update, and this function
        // should read that and update the transform, as I do with everything else. And moving it
        // to graphics too

        let mut sat_body_transform  = satellite_query.single_mut();

        sat_body_transform.rotate( Quat::from_rotation_z( spacecraft_parameters.rpm.value as f32 / 60.0 * time.delta_seconds()) ); 

    }

    fn update_sail_rotation (
        time:                   Res<Time>, 
        spacecraft_parameters:  Res<resources::SpacecraftParameters>, 
        //mut esail_query:        Query<&mut Transform, With<components::ESail>>,
        mut esail_query:        Query<(&mut Transform, &components::ESail)>,
        ){

        //let mut esail_transform     = esail_query.single_mut();
        let (mut esail_transform, mut esail)     = esail_query.single_mut();

        esail_transform.rotate( Quat::from_rotation_z( spacecraft_parameters.rpm.value as f32 / 60.0 * time.delta_seconds()) ); 

        // Not working, why? Do children not rotate with the parent?
        // It's rotating over itself!
        
        // I don't need it to rotate, it doesn't matter actually. What I need is to make it orbit around the center.
        
        //println!("{}", esail.distance_to_center());


        }

    /// Simulation proper
    fn verlet_simulation(
        time:                       Res<Time>, 
        esail_query:                Query<&components::ESail>,  // Should I add information about the pivot to ESail?
        solar_wind_parameters:      Res<resources::SolarWindParameters>,
        spacecraft_parameters:      Res<resources::SpacecraftParameters>,
        mut verlet_query:           Query<&mut components::VerletObject>,
        mut simulation_parameters:  ResMut<resources::SimulationParameters>,
        ) {

        let esail = esail_query.single();

        let timesteps = timestep_calculation(&time, &mut simulation_parameters);

        for _ in 0..timesteps { 

            // VERLET INTEGRATION: Forces are calculated and applied for each esail element

            for element in esail.elements.iter() {  // Iterating over esail elements, in order.

                let mut verlet_object = verlet_query.get_mut(*element).expect("No sail element found");

                if verlet_object.is_deployed {  // Needed?
                    verlet_integration(&mut simulation_parameters, &mut verlet_object, &spacecraft_parameters, &solar_wind_parameters);
                }
            }

            // CONSTRAINT LOOP. All operations in pixels, I'm pretty sure.

            for _ in 0..simulation_parameters.iterations {

                for index in 0..esail.elements.len() {  

                    // Distance between element and preceding element (in pixels). 
                    let pixels_between_elements = esail.pixels_between_elements(index, &verlet_query);  // The return is a DVec3 now

                    // Desired distance between elements (in pixels)
                    let desired_pixels_between_elements = spacecraft_parameters.segment_length().value * simulation_parameters.pixels_per_meter as f64;

                    // If difference is zero then I can skip all the rest, right? Perfect spot for an early return.

                    let difference = if pixels_between_elements.length() > 0.0 {
                        (desired_pixels_between_elements - pixels_between_elements.length()) / pixels_between_elements.length()
                    } else {
                        0.0
                    };

                    let correction_vector = if index > 0 {
                        pixels_between_elements.mul(0.5 * difference)
                    } else {
                        pixels_between_elements.mul(difference)
                    };

                    // UPDATING POSITIONS
                    
                    let mut current_verlet_object = verlet_query.get_mut(esail.elements[index]).expect("No previous sail element found");

                    current_verlet_object.correct_current_coordinates(correction_vector);

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
        mass_query:     Query<(&Transform, &components::Mass), Without<components::CenterOfMass>>,
        mut com_query:  Query<&mut Transform, With<components::CenterOfMass>>, 
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
/// This needs a rewrite using vectors
fn verlet_integration(
    simulation_parameters:  &mut ResMut<resources::SimulationParameters>,
    verlet_object:          &mut components::VerletObject,
    spacecraft_parameters:  &Res<resources::SpacecraftParameters>,
    solar_wind_parameters:  &Res<resources::SolarWindParameters>,
    ){

    // CALCULATION OF VELOCITIES

    let current_position_x  = verlet_object.current_x;
    let current_position_y  = verlet_object.current_y;
    let current_position_z  = verlet_object.current_z;

    let previous_position_x = verlet_object.previous_x;
    let previous_position_y = verlet_object.previous_y;
    let previous_position_z = verlet_object.previous_z;

    // Test
    //let prev_positions = verlet_object.previous_coordinates();
    //println!("{:?}", prev_positions[0]);

    // Maybe I shouldn't call these velocities, even if they are proportional to that.
    let velocity_x = current_position_x - previous_position_x;
    let velocity_y = current_position_y - previous_position_y;
    let velocity_z = current_position_z - previous_position_z;


    // FORCES
    //
    // Improvements:
    // * Each element will need to calculate its own centrifugal force!
    // * Forces should be vectors instead of going over one axis like they do now

    // X AXIS: Centrifugal force

    let distance_to_center = (current_position_x * current_position_x + current_position_y * current_position_y).sqrt();

    let angular_velocity = spacecraft_parameters.rpm * consts::PI / 30.0;   // What was this 30.0 for?

    let acceleration_x = distance_to_center * angular_velocity * angular_velocity;

    let next_position_x = current_position_x + velocity_x + acceleration_x.value * simulation_parameters.timestep * simulation_parameters.timestep;

    // Y̶ Z AXIS: Coulomb drag
    
    let force_per_segment = coulomb_force_per_meter(&solar_wind_parameters, &spacecraft_parameters) * spacecraft_parameters.segment_length();

    // Should I pass the mass from the mass query instead? (They should be exactly the same)
    let acceleration_z = - force_per_segment / spacecraft_parameters.segment_mass(); 

    //println!("{}: {:?}", "Force per segment", force_per_segment);    
    //println!("{}: {:?}", "Total force", force_per_segment * spacecraft_parameters.wire_resolution.value * spacecraft_parameters.wire_length.value);
    //println!("-------------------------");

    //let next_position_y = current_position_y + velocity_y + acceleration_y.value * simulation_parameters.timestep * simulation_parameters.timestep;
    let next_position_z = current_position_z + velocity_z + acceleration_z.value * simulation_parameters.timestep * simulation_parameters.timestep;
    
    // Starting to think that the bending moment should go here too.

    // UPDATING OBJECT POSITION

    // Previous position is forgotten,
    
    // current position becomes previous position,
    verlet_object.previous_x = current_position_x;
    //verlet_object.previous_y = current_position_y;
    verlet_object.previous_z = current_position_z;

    // and next position becomes current position.
    verlet_object.current_x = next_position_x;
    //verlet_object.current_y = next_position_y;
    verlet_object.current_z = next_position_z;
}

// From janhunen2007, equation 8. Corroborate all the results. And recheck the equations too.
#[allow(non_snake_case)]
pub fn coulomb_force_per_meter( 
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
