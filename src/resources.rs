use bevy::prelude::*;

//pub const PIXELS_PER_METER:         i32 = 100;

#[derive(Resource)]
pub struct SimulationParameters {
    pub iterations:     i32,    // Number of constraint iterations per timestep.
    pub timestep:       f32,    // Timestep for the physics simulation, in seconds.
    pub leftover_time:  f32,    // Unused time from the previous simulation loop.
    pub debug:          bool,   // Toggle for printing debug information to console.
    pub com_visibility: bool,   // Toggle for showing/hiding the center of mass.
}

impl Default for SimulationParameters {
    fn default() -> SimulationParameters {
        SimulationParameters {
            iterations:     60,
            timestep:       1.0/60.0,   // In seconds (right?)
            leftover_time:  0.0,
            debug:          false,
            com_visibility: false,
        }
    }
}

// Maybe it would make more sense as an entity with this as a component
#[derive(Resource)]
pub struct SpacecraftParameters {
    pub rpm:                i32,
    //pub wire_length:        f32,    // meters
    //pub wire_resolution:    f32,    // divisions per meter
    pub wire_potential:     f32,
    pub resting_distance:   f32,    // In pixels for now, go metric as soon as you can.
    pub number_of_elements: i32,    // Should instead choose a length and a resolution but ok
}

impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {
        SpacecraftParameters {
            rpm:                0,
            //wire_length:        1.0,    
            //wire_resolution:    10.0,
            wire_potential:     0.0,
            resting_distance:   20.0,
            number_of_elements: 20,
        }
    }
}
