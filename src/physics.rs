// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use crate::{ components, resources };

pub const PHYSICS_TIMESTEP: f32 = 1.0/60.0; // seconds
pub const ITERATIONS:       i32 = 100;    // One seems to be enough now

pub const DEBUG:            bool = false;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(verlet_simulation)
            .add_system(transform_update) // Does this always work after the previous system is finished?
            .insert_resource(resources::SimulationParameters{timestep: PHYSICS_TIMESTEP, ..Default::default()});
    }
}

/// Calculates how many timesteps should happen in the current frame, considering any potential unspent time from the previous frame.
fn timestep_calculation(
    time: &Res<Time>,
    sim_params: &mut ResMut<resources::SimulationParameters>,
    ) -> i32 {

    // Time since last update plus leftover time from previous frame
    let elapsed_time = time.delta_seconds() + sim_params.leftover_time; 

    // Number of timesteps that should be performed during this frame
    let timesteps = (elapsed_time / sim_params.timestep).floor() as i32; 

    // Calculating and storing leftover time, to be used in the next frame
    let leftover_time = elapsed_time - timesteps as f32 * sim_params.timestep;
    sim_params.leftover_time = leftover_time;

    return timesteps;
}

/// Updates the position of a verlet object
fn verlet_integration(
    verlet_object:  &mut components::VerletObject,
    sim_params:     &mut ResMut<resources::SimulationParameters>,
    ){

    let current_position_x  = verlet_object.current_x;
    let current_position_y  = verlet_object.current_y;

    let previous_position_x = verlet_object.previous_x;
    let previous_position_y = verlet_object.previous_y;

    // Applying accelerations

    let velocity_x = current_position_x - previous_position_x;
    let velocity_y = current_position_y - previous_position_y;

    let next_position_x = current_position_x + velocity_x + sim_params.acceleration_x * sim_params.timestep * sim_params.timestep;
    let next_position_y = current_position_y + velocity_y + sim_params.acceleration_y * sim_params.timestep * sim_params.timestep;

    // Updating verlet object:

    // Previous position is forgotten,
    
    // current position becomes previous position,
    verlet_object.previous_x = current_position_x;
    verlet_object.previous_y = current_position_y;

    // and next position becomes current position.
    verlet_object.current_x = next_position_x;
    verlet_object.current_y = next_position_y;
}

/// Simulation proper
fn verlet_simulation(
    time: Res<Time>,
    esail: Res<resources::ESail>,
    mut sim_params: ResMut<resources::SimulationParameters>,
    //mut sail_query: Query<(&components::SailElement, &mut components::VerletObject)>,
    mut sail_query: Query<&mut components::VerletObject, With<components::SailElement>>,
    //mut sail_query: Query<&mut components::VerletObject>,
    ) {

    // CALCULATION OF TIMESTEPS FOR THE CURRENT FRAME

    let timesteps = timestep_calculation(&time, &mut sim_params);

    if DEBUG { println!("New frame ------------------"); }

    for _ in 0..timesteps { 

        if DEBUG { println!("New timestep ---------------"); }

        // SIMULATION LOOP

        // Iterating over esail elements, in order.
        for element in esail.elements.iter() {

            // Getting information about the current sail element
            //let (sail_element,  mut verlet_object) = sail_query.get_mut(*element).expect("No sail element found");
            let mut verlet_object = sail_query.get_mut(*element).expect("No sail element found");

            //if sail_element.is_deployed {
            //    // Updating the values of the verlet object
            //    verlet_integration(&mut verlet_object, &mut sim_params);
            //}

            if verlet_object.is_deployed {
                verlet_integration(&mut verlet_object, &mut sim_params);
            }
        }

        // CONSTRAINT LOOP

        for _ in 0..ITERATIONS {

            if DEBUG { println!("New constraint iteration ---"); }

            let mut verlet_combinations = sail_query.iter_combinations_mut::<2>();

            while let Some([mut first_verlet, mut second_verlet]) = verlet_combinations.fetch_next() {

                let mut first_verlet_x = first_verlet.current_x;
                let mut first_verlet_y = first_verlet.current_y;
                
                let mut second_verlet_x = second_verlet.current_x;
                let mut second_verlet_y = second_verlet.current_y;

                // Calculating distance between elements

                let diff_x = first_verlet_x - second_verlet_x;
                let diff_y = first_verlet_y - second_verlet_y;
                let distance_between_elements = (diff_x * diff_x + diff_y * diff_y).sqrt();
                
                let mut difference = 0.0;

                if distance_between_elements > 0.0 {
                    // Don't get this formula really
                    difference = (distance_between_elements - esail.resting_distance) / distance_between_elements;
                }

                let correction_x = diff_x * difference;
                let correction_y = diff_y * difference;

                // Updating positions

                if first_verlet.is_deployed {
                    first_verlet.current_x -= correction_x;
                    first_verlet.current_y -= correction_y;
                }

                if second_verlet.is_deployed {
                    second_verlet.current_x += correction_x;
                    second_verlet.current_y += correction_y;
                }
            }

        }
    }
}

fn transform_update(
    mut sail_query: Query<(&components::VerletObject, &mut Transform)>,
    ){
    
    for (verlet_object, mut transform) in sail_query.iter_mut() {
        transform.translation.x = verlet_object.current_x;
        transform.translation.y = verlet_object.current_y;
    }
} 
