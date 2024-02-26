use bevy::prelude::*;

use uom::si::f64 as quantities;
use uom::si::*;

use uom::lib::marker::PhantomData;

// Maybe a constants.rs could contain these
pub const M_PROTON:  quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 1.672e-27};
pub const Q_E:       quantities::ElectricCharge = quantities::ElectricCharge {dimension: PhantomData, units: PhantomData, value: 1.602_176_634_E-19};  // Is this in Coulombs, you sure?
pub const EPSILON_0: quantities::ElectricPermittivity = quantities::ElectricPermittivity {dimension: PhantomData, units: PhantomData, value: 8.854e-12};


#[derive(Resource)]
pub struct SimulationParameters {
    pub iterations:         i32,    // Number of constraint iterations per timestep.
    pub timestep:           f64,    // Timestep for the physics simulation, in seconds. Should be an uom quantity, right??
    pub timestep_s:         quantities::Time,   // Update everything to uom seconds later
    pub leftover_time:      f64,    // Unused time from the previous simulation loop.
    pub debug:              bool,   // Toggle for printing debug information to console.
    pub com_visibility:     bool,   // Toggle for showing/hiding the center of mass.
    pub axes_visibility:    bool,
    pub pixels_per_meter:   i32,
}

impl Default for SimulationParameters {
    fn default() -> SimulationParameters {
        SimulationParameters {
            iterations:         60,
            timestep:           1.0/60.0,   // In seconds (right?)
            timestep_s:         quantities::Time::new::<time::second>(1.0/60.0),
            leftover_time:      0.0,
            debug:              false,
            com_visibility:     false,
            axes_visibility:    true,
            pixels_per_meter:   500,
        }
    }
}

