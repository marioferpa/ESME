// Move the simulation plugin and the resource. Leave physics.rs only with use position_vector, use etc
use bevy::prelude::*;

mod verlet_simulation;
//mod new_verlet_simulation;
mod voltage;

pub struct SimulationPlugin;

// TODO Work for today: understanding why past me decided to make a new esail
// and a new verlet sim. For now I see that the sail is a vector of VerletObject
// instead of Entities

// I think that what happened is that I started working on a new, simpler Esail that contained
// VerletObject direcly, instead of entities, and that was half done when I paused development.
// And then, without touching the code, I started to think that I didn't really need to model each
// verlet as an entity, but rather the esail could be an entity, and contain a list of verlets that
// you operate on. And that in turn would make ECS not that useful, maybe.
//
// Let's branch out, destroy new_esail, and make new_esail instead

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
