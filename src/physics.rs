// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use std::f32::consts;
use bevy::prelude::*;
use crate::{ components, parameters };

use uom::si::length::meter;
use uom::fmt::DisplayStyle::Description;

// Values for Coulomb interaction

//const K:            f32 = 3.09;         // Don't know what it is    
//const T_E:          f32 = 12.0;         // (eV) Solar wind electron temperature at 1AU



pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(parameters::SimulationParameters{..Default::default()})
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
        spacecraft_parameters: Res<parameters::SpacecraftParameters>,
        mut electrical_query: Query<&mut components::ElectricallyCharged>,
        ) {

        for mut electrical_element in electrical_query.iter_mut() {
            electrical_element.potential = spacecraft_parameters.wire_potential_V;
            //println!("{}", spacecraft_parameters.wire_potential_V);
        }
    }

    /// Simulation proper
    fn verlet_simulation(
        time: Res<Time>, esail_query: Query<&components::ESail>,
        mut sim_params: ResMut<parameters::SimulationParameters>,
        spacecraft_parameters: Res<parameters::SpacecraftParameters>,
        solar_wind_parameters: Res<parameters::SolarWindParameters>,
        mut sail_query: Query<(&mut components::VerletObject, &components::Mass), With<components::SailElement>>,
        ) {

        let esail = esail_query.single();

        let timesteps = timestep_calculation(&time, &mut sim_params);

        //if sim_params.debug { println!("New frame ------------------"); }

        for _ in 0..timesteps { 

          //  if sim_params.debug { println!("New timestep ---------------"); }

            // VERLET INTEGRATION
            // Forces are calculated and applied for each esail element

            for element in esail.elements.iter() {  // Iterating over esail elements, in order.

                let (mut verlet_object, _) = sail_query.get_mut(*element).expect("No sail element found");

                if verlet_object.is_deployed {
                    //verlet_integration(&mut sim_params, &mut verlet_object, &spacecraft_parameters);
                    verlet_integration(&mut sim_params, &mut verlet_object, &spacecraft_parameters, &solar_wind_parameters);
                }
            }

            // CONSTRAINT LOOP
            // Final position is corrected taking into account the constraints of the system

            for _ in 0..sim_params.iterations {

                //if sim_params.debug { println!("New constraint iteration ---"); }

                for (index, sail_element) in esail.elements.iter().enumerate().skip(1) {    // Iterating over the sail elements in order. Skips the first.
                                                                                            // Needed if I'm already checking for deployment?

                    // Information from current element
                    let (current_verlet_object, _) = sail_query.get(*sail_element).expect("No previous sail element found");

                    let current_element_x = current_verlet_object.current_x;
                    let current_element_y = current_verlet_object.current_y;

                    // Information from previous element
                    let prev_sail_element = esail.elements[index - 1];
                    let (prev_verlet_object, _) = sail_query.get(prev_sail_element).expect("No previous sail element found");

                    let prev_element_x = prev_verlet_object.current_x;
                    let prev_element_y = prev_verlet_object.current_y;

                    // Calculating distance between current sail element and previous element in the line
                    let diff_x = current_element_x - prev_element_x;
                    let diff_y = current_element_y - prev_element_y;
                    let distance_between_elements = (diff_x * diff_x + diff_y * diff_y).sqrt();

                    //if sim_params.debug {
                    //    println!("Index: {} | Distance between elements: {}", index, distance_between_elements);
                    //}

                    let mut difference = 0.0;

                    if distance_between_elements > 0.0 {
                        //difference = (spacecraft_parameters.resting_distance - distance_between_elements) / distance_between_elements;
                        difference = (esail.resting_distance - distance_between_elements) / distance_between_elements;
                    }

                    // This shouldn't be .5 if one object is not deployed, although I believe it tends to the correct spot anyways.
                    let correction_x = diff_x * 0.5 * difference;
                    let correction_y = diff_y * 0.5 * difference;

                    // UPDATING POSITIONS
                    // Yes, I'm querying again, can't find a cleaner way to do it.
                    
                    let (mut current_verlet_object, _) = sail_query.get_mut(*sail_element).expect("No previous sail element found");
                    
                    if current_verlet_object.is_deployed {
                        current_verlet_object.current_x += correction_x;
                        current_verlet_object.current_y += correction_y;
                    }

                    let (mut prev_verlet_object, _) = sail_query.get_mut(prev_sail_element).expect("No previous sail element found");
                    
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
        mut sail_query: Query<(&components::VerletObject, &mut Transform)>,
        ){
        
        for (verlet_object, mut transform) in sail_query.iter_mut() {
            transform.translation.x = verlet_object.current_x;
            transform.translation.y = verlet_object.current_y;

        }
    } 

    /// Updates position and visibility of the center of mass
    fn update_center_of_mass(
        sim_params:     Res<parameters::SimulationParameters>,
        mass_query:     Query<(&Transform, &components::Mass), Without<components::CenterOfMass>>,
        mut com_query:  Query<(&mut Transform, &mut Visibility), With<components::CenterOfMass>>, 
        ){

        let mut total_mass:     f32 = 0.0;
        let mut center_mass_x:  f32 = 0.0;
        let mut center_mass_y:  f32 = 0.0;

        for (transform, object_mass) in mass_query.iter() {
            total_mass += object_mass.0;
            center_mass_x += transform.translation.x * object_mass.0;
            center_mass_y += transform.translation.y * object_mass.0;
        }

        if sim_params.debug {
            println!("Total mass: {} | Center of mass: ({},{})", total_mass, center_mass_x, center_mass_y);
        }

        let (mut com_transform, mut com_visibility) = com_query.single_mut();

        com_transform.translation.x = center_mass_x;
        com_transform.translation.y = center_mass_y;

        com_visibility.is_visible = sim_params.com_visibility;
    }
}


/// Updates the position of a verlet object
fn verlet_integration(
    sim_params:             &mut ResMut<parameters::SimulationParameters>,
    verlet_object:          &mut components::VerletObject,
    spacecraft_parameters:  &Res<parameters::SpacecraftParameters>,
    solar_wind_parameters:  &Res<parameters::SolarWindParameters>,
    ){

    // CALCULATION OF VELOCITIES

    let current_position_x  = verlet_object.current_x;
    let current_position_y  = verlet_object.current_y;

    let previous_position_x = verlet_object.previous_x;
    let previous_position_y = verlet_object.previous_y;

    let velocity_x = current_position_x - previous_position_x;
    let velocity_y /* wait what */ = current_position_y - previous_position_y;


    // FORCES

    // X AXIS: Centrifugal force

    let distance_to_center = (current_position_x * current_position_x + current_position_y * current_position_y).sqrt();

    let angular_velocity = spacecraft_parameters.rpm as f32 * consts::PI / 30.0;

    let acceleration_x = distance_to_center * angular_velocity * angular_velocity;

    let next_position_x = current_position_x + velocity_x + acceleration_x * sim_params.timestep * sim_params.timestep;

    // Y AXIS: Coulomb drag

    let acceleration_y = spacecraft_parameters.wire_potential_V; // I'm gonna make it just proportional to the voltage for now.

    // Not used yet
    let coulomb_force = coulomb_force(&solar_wind_parameters, &spacecraft_parameters);

    let next_position_y = current_position_y + velocity_y + acceleration_y * sim_params.timestep * sim_params.timestep;
    
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

// TESTING
// From janhunen2007, equation 8. Corroborate all the results. And recheck the equations too.
#[allow(non_snake_case)]
fn coulomb_force(
    solar_wind: &Res<parameters::SolarWindParameters>, 
    spacecraft: &Res<parameters::SpacecraftParameters>,
    ) -> f32 {

    let r_w = spacecraft.wire_radius;
    println!("r_w (m): {}", r_w.into_format_args(meter, Description));

    // First: r_0, distance at which the potential vanishes
    //let r0_numerator:   f32 = parameters::EPSILON_0 * solar_wind.T_e;
    //let r0_denominator: f32 = solar_wind.n_0 * parameters::Q_E * parameters::Q_E; 

    //let r_0 =  2.0 * (r0_numerator / r0_denominator).sqrt();
    //println!("r_o (m): {}", r_0);

    //// Second: r_s, stopping distance of protons
    //let exp_numerator = parameters::M_PROTON * solar_wind.velocity * solar_wind.velocity * (r_0 / r_w).ln();
    //let exp_denominator = parameters::Q_E * spacecraft.wire_potential_V; 

    //let r_s_denominator = ((exp_numerator / exp_denominator).exp() - 1.0).sqrt();

    //let r_s = r_0 / r_s_denominator;
    //println!("{}", r_s);

    //// Third: force per unit length
    //let K = 3.09;   // Empirical, from Monte Carlo sims, I need to calculate this myself somehow.

    //let force_per_length = r_s * K * parameters::M_PROTON * solar_wind.n_0 * solar_wind.velocity * solar_wind.velocity;
    //println!("{}", force_per_length);

    //return force_per_length;

    return 0.0;
}

/// Calculates how many timesteps should happen in the current frame, considering any potential unspent time from the previous frame.
fn timestep_calculation(
    time: &Res<Time>,
    sim_params: &mut ResMut<parameters::SimulationParameters>,
    ) -> i32 {

    let elapsed_time = time.delta_seconds() + sim_params.leftover_time; // Elapsed time + leftover time from previous frame

    let timesteps = (elapsed_time / sim_params.timestep).floor() as i32; // Number of timesteps for the current frame

    let leftover_time = elapsed_time - timesteps as f32 * sim_params.timestep;  // Leftover time saved for next frame
    sim_params.leftover_time = leftover_time;

    return timesteps;
}
