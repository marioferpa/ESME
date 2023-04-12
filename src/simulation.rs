// Move the simulation plugin and the resource. Leave physics.rs only with use position_vector, use etc
use bevy::prelude::*;

mod verlet_simulation;
mod voltage;

pub struct SimulationPlugin;   // Plugins are structs, therefore they can hold data!

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                verlet_simulation::verlet_simulation
                    .label("VerletSimulation")
                    .after("Deploy")    // I don't know if this is working
            )
            .add_system(voltage::update_esail_voltage)
            //.add_system(Self::update_center_of_mass)        // Updates position of the center of mass
            ;
    }
}
