use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationParameters {
    pub iterations:         i32,    // Number of constraint iterations per timestep.
    pub timestep:           f32,    // Timestep for the physics simulation, in seconds.
    pub leftover_time:      f32,    // Unused time from the previous simulation loop.
    pub debug:              bool,   // Toggle for printing debug information to console.
    pub com_visibility:     bool,   // Toggle for showing/hiding the center of mass.
    pub pixels_per_meter:   i32,
}

impl Default for SimulationParameters {
    fn default() -> SimulationParameters {
        SimulationParameters {
            iterations:         60,
            timestep:           1.0/60.0,   // In seconds (right?)
            leftover_time:      0.0,
            debug:              false,
            com_visibility:     false,
            pixels_per_meter:   500,
        }
    }
}

#[derive(Resource)]
pub struct SpacecraftParameters {
    pub rpm:                i32,
    pub wire_length:        f32,    // meters
    pub wire_resolution:    f32,    // divisions per meter
    pub wire_potential:     f32,
}

impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {
        SpacecraftParameters {
            rpm:                0,
            wire_length:        1.0,    // meters
            wire_resolution:    25.0,   // divisions per meter
            wire_potential:     0.0,
        }
    }
}
