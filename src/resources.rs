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
    pub iterations:         i32,    
    pub timestep:           quantities::Time,
    pub leftover_time:      f64,
    pub debug:              bool,
    pub com_visibility:     bool,
    pub axes_visibility:    bool,
    pub pixels_per_meter:   i32,
}

// timestep and timestep_s? Which one am I using?

impl Default for SimulationParameters {
    fn default() -> SimulationParameters {
        SimulationParameters {
            //iterations:         60,
            iterations:         100,
            timestep:           quantities::Time::new::<time::second>(1.0/60.0),
            leftover_time:      0.0,
            debug:              false,
            com_visibility:     false,
            axes_visibility:    true,
            pixels_per_meter:   500,
        }
    }
}

