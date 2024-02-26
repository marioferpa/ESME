// Move the simulation plugin and the resource. Leave physics.rs only with use position_vector, use etc
use bevy::prelude::*;

mod verlet_simulation;
mod new_verlet_simulation;
mod voltage;

pub struct SimulationPlugin;   // Plugins are structs, therefore they can hold data!

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, (
                    verlet_simulation::verlet_simulation,
                    new_verlet_simulation::new_verlet_simulation,
                    voltage::update_esail_voltage
                )
            )
        ;
    }
}
