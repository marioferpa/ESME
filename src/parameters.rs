use bevy::prelude::*;

// UOM package, for physical units
use uom::si::f32::*;
use uom::si::energy::electronvolt;

// Maybe a constants.rs could contain these
pub const M_PROTON:     f32 = 1.672e-27;    // (kg) Is the scientific notation alright in Rust? Wow, love it
pub const EPSILON_0:    f32 = 8.854e-12;    // (Fm^-1) Vacuum permitivity
pub const Q_E:          f32 = 1.602e-19;    // (C) Electron charge

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
#[allow(non_snake_case)]
pub struct SpacecraftParameters {
    pub rpm:                i32,
    pub wire_length_m:      f32,    // meters
    pub wire_radius_m:      f32,    // meters
    pub wire_resolution:    f32,    // divisions per meter
    pub wire_potential_V:   f32,
}

impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {
        SpacecraftParameters {
            rpm:                0,
            wire_length_m:      1.0,    // meters
            wire_radius_m:      0.01,   // meters
            wire_resolution:    25.0,   // divisions per meter
            wire_potential_V:   0.0,
        }
    }
}

#[derive(Resource)]
#[allow(non_snake_case)]
pub struct SolarWindParameters {
    pub n_0:        f32,    // (cm^-3, careful) Undisturbed solar wind electron density
    pub velocity:   f32,
    pub T_e:        f32,    // (eV) Solar wind electron temperature at 1AU
    pub test:       Energy,
}

impl Default for SolarWindParameters {
    fn default() -> SolarWindParameters {
        SolarWindParameters {
            n_0:        7.3,    // cm^-3, careful
            velocity:   4.0e5,  // m/s (from google, can't find it in the paper)
            T_e:        12.0,   // eV
            test:       Energy::new::<electronvolt>(12.0),
        }
    }
}

