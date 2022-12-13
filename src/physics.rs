// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use crate::{ components, resources };

pub const ACCELERATION_X:   f32 = 10000.0;    // pixel*s⁻² ?
pub const ACCELERATION_Y:   f32 = 5000.0;   // pixel*s⁻² ?
pub const PHYSICS_TIMESTEP: f32 = 1.0/60.0; // seconds
const ITERATIONS:           i32 = 1;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_positions)
            .insert_resource(resources::SimulationParameters{timestep: PHYSICS_TIMESTEP, ..Default::default()});
    }
}

impl PhysicsPlugin {

    fn update_positions(
        time: Res<Time>,
        esail: Res<resources::ESail>,
        mut sim_params: ResMut<resources::SimulationParameters>,
        mut sail_query: Query<(&components::SailElement, &mut Transform, &mut components::VerletElement)>
        ) {

        // CALCULATION OF NUMBER OF TIMESTEPS

        // Bevy's timestep may not be consistent, and instead of trying to control it, I choose the
        // timestep that I want, calculate as many timesteps as possible, and add the leftover time
        // to the next timestep's physics loop.

        // Time since last update plus leftover time from previous frame
        let elapsed_time = time.delta_seconds() + sim_params.leftover_time;

        // Number of timesteps that should be calculated during this update
        let timesteps = (elapsed_time / sim_params.timestep).floor() as i32; 
        
        // Recalculation of leftover time for next update
        let leftover_time = elapsed_time - timesteps as f32 * sim_params.timestep;
        sim_params.leftover_time = leftover_time;

        println!("New frame ------------------");

        for _ in 0..timesteps { 

            println!("New timestep ---------------");

            // I think I need two loops. One to calculate where each element would end up if moving
            // freely. And then another, again over all sail elements but also over multiple
            // iterations, checking each element and the previous and applying the constraints.

            // SIMULATION LOOP

            // Iterating over esail elements, in order. The first one is skipped because it's static
            for sail_element in esail.elements.iter().skip(1) {

                // Getting coordinates of the current sail element
                let (_sail_element, mut transform, mut verlet_element) = sail_query.get_mut(*sail_element).expect("No sail element found");

                let position_x = transform.translation.x;
                let position_y = transform.translation.y;

                // APPLYING (gravitational for now) ACCELERATION

                let velocity_x = position_x - verlet_element.previous_x;
                let velocity_y = position_y - verlet_element.previous_y;

                let next_position_x = position_x + velocity_x + ACCELERATION_X * sim_params.timestep * sim_params.timestep;
                let next_position_y = position_y + velocity_y + ACCELERATION_Y * sim_params.timestep * sim_params.timestep;

                verlet_element.previous_x = transform.translation.x;
                verlet_element.previous_y = transform.translation.y;

                verlet_element.current_x = next_position_x;
                verlet_element.current_y = next_position_y;

                transform.translation.x = verlet_element.current_x;
                transform.translation.y = verlet_element.current_y;

            }

            // CONSTRAINT LOOP

            for _ in 0..ITERATIONS {
                println!("New constraint iteration ---");

                // Iterate again over all the objects
                for (index, sail_element) in esail.elements.iter().enumerate().skip(1) {

                    // Getting coordinates of the previous sail element in line
                    let prev_sail_element = esail.elements[index - 1];

                    let (_prev_element, prev_element_transform, _prev_verlet_element) = sail_query.get(prev_sail_element).expect("No previous sail element found");

                    let prev_element_x = prev_element_transform.translation.x;
                    let prev_element_y = prev_element_transform.translation.y;

                    // Getting coordinates of the current sail element
                    let (_sail_element, mut transform, mut verlet_element) = sail_query.get_mut(*sail_element).expect("No sail element found");
                
                    // Calculating distance between current sail element and previous element in the line
                    let diff_x = verlet_element.current_x - prev_element_x;
                    let diff_y = verlet_element.current_y - prev_element_y;
                    let distance_between_elements = (diff_x * diff_x + diff_y * diff_y).sqrt();

                    println!("Index: {} | Distance between elements: {}", index, distance_between_elements);

                    let mut difference = 0.0;

                    if distance_between_elements > 0.0 {
                        // Don't get this formula really
                        difference = (distance_between_elements - esail.resting_distance) / distance_between_elements;
                    }

                    let correction_x = diff_x * difference;
                    let correction_y = diff_y * difference;

                    // Updating positions

                    verlet_element.previous_x = verlet_element.current_x;
                    verlet_element.previous_y = verlet_element.current_y;

                    transform.translation.x = verlet_element.current_x - correction_x;
                    transform.translation.y = verlet_element.current_y - correction_y;
                }
            }
        }
    }
}
