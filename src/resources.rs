use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationParameters {
    pub timestep:       f32,    // Timestep for the physics simulation, in seconds
    pub leftover_time:  f32,    // Unused time from the previous simulation loop
}

impl Default for SimulationParameters {
    fn default() -> SimulationParameters {
        SimulationParameters {
            timestep:       1.0/60.0,
            leftover_time:  0.0,
        }
    }
}

// Maybe it would make more sense as an entity with this as a component
#[derive(Resource)]
// Inspired by https://stackoverflow.com/questions/74031066/is-there-a-way-to-do-complex-queries-in-bevy-ecs
pub struct ESail {
    pub elements: Vec<Option<Entity>>, // Option so that this can be empty?
}
