// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
//use std::time;
use crate::{ components };

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_positions)
            .insert_resource(SimulationParameters{timestep: 1.0/60.0, leftover_time: 0.0});
    }
}

#[derive(Resource)]
pub struct SimulationParameters {
    timestep:       f32, // Timestep for the physics simulation, in seconds
    leftover_time:  f32  // Unused time from the previous simulation loop
}

impl PhysicsPlugin {

    fn update_positions(
        time: Res<Time>,
        mut sim_params: ResMut<SimulationParameters>,
        mut sail_query: Query<(&components::SailElement, &mut Transform)>
        ) {

        let _velocity_x: f32 = 0.0;    // unclear units for now 
        let velocity_y: f32 = -50.0;

        //println!("{:?}", sim_params.leftover_time);

        // Time since last update plus leftover time
        let elapsed_time = time.delta_seconds() + sim_params.leftover_time;

        // Number of timesteps that should be calculated during this update
        let timesteps = (elapsed_time / sim_params.timestep).floor() as i32; 
        
        // Recalculation of leftover time for next update
        let leftover_time = elapsed_time - timesteps as f32 * sim_params.timestep;
        sim_params.leftover_time = leftover_time;

        // Simulation loop, for whatever many timesteps are needed
        for _ in 0..timesteps { // Make sure that this is not skipping one or something
            for (_element, mut transform) in sail_query.iter_mut() {
                transform.translation.y += velocity_y * sim_params.timestep;
                //println!("{:?}", transform.translation.y);
            }
        }
    }
}
