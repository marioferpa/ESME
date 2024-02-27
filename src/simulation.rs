// Move the simulation plugin and the resource. Leave physics.rs only with use position_vector, use etc
use bevy::prelude::*;

mod verlet_simulation;
//mod new_verlet_simulation;
mod voltage;

pub struct SimulationPlugin;

// TODO Work for today: understanding why past me decided to make a new esail
// and a new verlet sim. For now I see that the sail is a vector of VerletObject
// instead of Entities

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, (
                    verlet_simulation::verlet_simulation,
                    //new_verlet_simulation::new_verlet_simulation,
                    voltage::update_esail_voltage
                )
            )
        ;
    }
}
