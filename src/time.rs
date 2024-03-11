use bevy::prelude::*;

use uom::si::time::second;

use crate::{ resources };

/// Calculates how many timesteps should happen in the current frame,
/// considering any potential unspent time from the previous frame.
pub fn timestep_calculation (
    time: &Res<Time>,
    sim_params: &mut ResMut<resources::SimulationParameters>,
) -> i32 {

    let elapsed_time = time.delta_seconds() as f64 + sim_params.leftover_time;

    let timesteps = 
        (elapsed_time / sim_params.timestep.get::<second>()).floor() as i32;

    let leftover_time = 
        elapsed_time - timesteps as f64 * sim_params.timestep.get::<second>();

    sim_params.leftover_time = leftover_time;

    return timesteps;
}
