// Move the simulation plugin and the resource. Leave physics.rs only with use position_vector, use etc
use bevy::prelude::*;

//mod verlet_simulation;
mod new_verlet_simulation;
mod voltage;

pub struct SimulationPlugin;

// TODO Make solar wind affect the new sail. Next, try to introduce the spring
// recovery force 
// Maybe they have no voltage?

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, (
                    //verlet_simulation::verlet_simulation,
                    new_verlet_simulation::new_verlet_simulation,
                    //voltage::update_esail_voltage
                )
            )
        ;
    }
}
