use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationParameters {
    pub iterations:     i32,
    pub timestep:       f32,    // Timestep for the physics simulation, in seconds
    pub leftover_time:  f32,    // Unused time from the previous simulation loop
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub debug:          bool,
}

impl Default for SimulationParameters {
    fn default() -> SimulationParameters {
        SimulationParameters {
            iterations:     10,
            timestep:       1.0/60.0,
            leftover_time:  0.0,
            acceleration_x: 0.0,
            acceleration_y: 0.0,
            debug:          false,
        }
    }
}

// Maybe it would make more sense as an entity with this as a component
#[derive(Resource)]
pub struct ESail {
    pub elements: Vec<Entity>,
    pub resting_distance: f32,  // In pixels for now, go metric as soon as you can.
}
