use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationParameters {
    pub iterations:     i32,
    pub timestep:       f32,    // Timestep for the physics simulation, in seconds
    pub leftover_time:  f32,    // Unused time from the previous simulation loop
    pub acceleration_y: f32,
    pub debug:          bool,
    pub com_visibility: bool,
}

impl Default for SimulationParameters {
    fn default() -> SimulationParameters {
        SimulationParameters {
            iterations:     10,
            timestep:       1.0/60.0,
            leftover_time:  0.0,
            acceleration_y: 0.0,
            debug:          false,
            com_visibility: true,
        }
    }
}

// Maybe it would make more sense as an entity with this as a component
#[derive(Resource)]
pub struct SpacecraftParameters {
    pub rpm:                i32,
    pub elements:           Vec<Entity>,    // Looks misplaced here. Number of elements would be alright
    pub wire_potential:     f32,
    pub resting_distance:   f32,  // In pixels for now, go metric as soon as you can.
}

impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {
        SpacecraftParameters {
            rpm:                0,
            elements:           Vec::new(),
            resting_distance:   10.0,
            wire_potential:     0.0,
        }
    }
}
