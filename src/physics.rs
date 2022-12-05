// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use crate::{ components, resources };

// Lowercase g is not rustacean
pub const ACCELERATION: f32 = -9.8;     // pixel*s⁻² ?
pub const PHYSICS_TIMESTEP:     f32 = 1.0/60.0; // seconds


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
        mut sail_query: Query<(&components::SailElement, &mut Transform, &mut components::CanMove)>
        ) {

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

        // Simulation loop, for however many timesteps are needed
        for _ in 0..timesteps { // Make sure that this is not skipping one or something

            // Iterating over esail elements, in order.
            for entity in esail.elements.iter() {
                // Make sure this iterates in order, and in the order you want!

                let (_element, mut transform, mut can_move) = sail_query.get_mut(*entity).expect("No sail element found");

                //println!("{:?}", transform.translation.y);
                
                // Applying acceleration

                let velocity_y = transform.translation.y - can_move.previous_y;

                let next_y = transform.translation.y + velocity_y + ACCELERATION * sim_params.timestep * sim_params.timestep;

                // Applying constraints

                // Updating positions

                can_move.previous_y = transform.translation.y;

                transform.translation.y = next_y;
            
            }
        }
    }
}
