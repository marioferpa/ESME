// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use crate::{ components, resources };

pub const PHYSICS_TIMESTEP: f32 = 1.0/60.0; // seconds
pub const ITERATIONS:       i32 = 100;    

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
    mut sail_query: Query<&mut components::VerletObject, With<components::SailElement>>,
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
            let mut verlet_object = sail_query.get_mut(*element).expect("No sail element found");

            if verlet_object.is_deployed {
                verlet_integration(&mut verlet_object, &mut sim_params);
            }
        }

        // CONSTRAINT LOOP

        for _ in 0..ITERATIONS {

            if DEBUG { println!("New constraint iteration ---"); }

            // Not working. I don't want all pairs of objects to be connected, I don't know what I was thinking.
            //let mut verlet_combinations = sail_query.iter_combinations_mut::<2>();
            //while let Some([mut first_verlet, mut second_verlet]) = verlet_combinations.fetch_next() {
            
            for (index, sail_element) in esail.elements.iter().enumerate().skip(1) {

                // Information from current element
                let current_verlet_object = sail_query.get(*sail_element).expect("No previous sail element found");

                let current_element_x = current_verlet_object.current_x;
                let current_element_y = current_verlet_object.current_y;

                // Information from previous element
                let prev_sail_element = esail.elements[index - 1];
                let prev_verlet_object = sail_query.get(prev_sail_element).expect("No previous sail element found");

                let prev_element_x = prev_verlet_object.current_x;
                let prev_element_y = prev_verlet_object.current_y;

                // Calculating distance between current sail element and previous element in the line
                let diff_x = current_element_x - prev_element_x;
                let diff_y = current_element_y - prev_element_y;
                let distance_between_elements = (diff_x * diff_x + diff_y * diff_y).sqrt();

                if DEBUG {
                    println!("Index: {} | Distance between elements: {}", index, distance_between_elements);
                }

                let mut difference = 0.0;

                if distance_between_elements > 0.0 {
                    // Don't get this formula really. Is it correct?
                    difference = (esail.resting_distance - distance_between_elements) / distance_between_elements;
                }

                // This shouldn't be .5 if one object is not deployed, although I believe it tends to the correct spot anyways.
                let correction_x = diff_x * 0.5 * difference;
                let correction_y = diff_y * 0.5 * difference;

                // UPDATING POSITIONS
                // Here's where, I think, I'll have to get the queries again. Or at least operate
                // on the prev query, that is still open, and then reopen the one for the current element.
                
                let mut current_verlet_object = sail_query.get_mut(*sail_element).expect("No previous sail element found");
                
                if current_verlet_object.is_deployed {
                    current_verlet_object.current_x += correction_x;
                    current_verlet_object.current_y += correction_y;
                }

                let mut prev_verlet_object = sail_query.get_mut(prev_sail_element).expect("No previous sail element found");
                
                if prev_verlet_object.is_deployed {
                    prev_verlet_object.current_x -= correction_x;
                    prev_verlet_object.current_y -= correction_y;
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
