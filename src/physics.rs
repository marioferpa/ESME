// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use crate::{ components };

// Lowercase g is not rustacean
pub const ACCELERATION: f32 = -9.8;  // pixel*s⁻² ?

#[derive(Resource)]
pub struct SimulationParameters {
    timestep:       f32, // Timestep for the physics simulation, in seconds
    leftover_time:  f32,  // Unused time from the previous simulation loop
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_positions)
            .insert_resource(SimulationParameters{timestep: 1.0/60.0, leftover_time: 0.0});
    }
}


impl PhysicsPlugin {

    fn update_positions(
        time: Res<Time>,
        mut sim_params: ResMut<SimulationParameters>,
        mut sail_query: Query<(&components::SailElement, &mut Transform, &mut components::CanMove)>
        ) {

        //let _velocity_x: f32 = 0.0;    // unclear units for now 
        //let velocity_y: f32 = -50.0;

        // Time since last update plus leftover time
        let elapsed_time = time.delta_seconds() + sim_params.leftover_time;

        // Number of timesteps that should be calculated during this update
        let timesteps = (elapsed_time / sim_params.timestep).floor() as i32; 
        
        // Recalculation of leftover time for next update
        let leftover_time = elapsed_time - timesteps as f32 * sim_params.timestep;
        sim_params.leftover_time = leftover_time;

        // Simulation loop, for whatever many timesteps are needed
        for _ in 0..timesteps { // Make sure that this is not skipping one or something
            for (_element, mut transform, mut can_move) in sail_query.iter_mut() {

                // Euler integration, right?
                //transform.translation.y += velocity_y * sim_params.timestep;
                //println!("{:?}", transform.translation.y);

                // Verlet integration
                //println!("{:?}", can_move.previous_y);

                let velocity_y = transform.translation.y - can_move.previous_y;

                let next_y = transform.translation.y + velocity_y + ACCELERATION * sim_params.timestep * sim_params.timestep;

                println!("{:?}", next_y - can_move.previous_y);

                can_move.previous_y = transform.translation.y;

                transform.translation.y = next_y;
                //println!("{:?}", next_y);
                println!("---------");



            }
        }
    }
}
