use bevy::prelude::*;
use std::f64::consts;

// UOM package, for physical units
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
    pub timestep:           f64,    // Timestep for the physics simulation, in seconds.
    pub leftover_time:      f64,    // Unused time from the previous simulation loop.
    pub debug:              bool,   // Toggle for printing debug information to console.
    pub com_visibility:     bool,   // Toggle for showing/hiding the center of mass.
    pub pixels_per_meter:   i32,
    pub three_dimensions:   bool,
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
            three_dimensions:   true,
        }
    }
}

#[derive(Resource)]
#[allow(non_snake_case)]
pub struct SpacecraftParameters {
    pub rpm:                quantities::Frequency,
    pub wire_length:        quantities::Length,
    pub wire_radius:        quantities::Length, 
    pub wire_density:       quantities::MassDensity,
    pub wire_potential:     quantities::ElectricPotential,
    pub wire_resolution:    quantities::LinearNumberDensity,
}

impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {
        SpacecraftParameters {
            rpm:                quantities::Frequency::new::<frequency::cycle_per_minute>(0.0),
            wire_length:        quantities::Length::new::<length::meter>(1.0),
            wire_radius:        quantities::Length::new::<length::micrometer>(10.0),
            wire_density:       quantities::MassDensity::new::<mass_density::gram_per_cubic_centimeter>(2.7),
            wire_potential:     quantities::ElectricPotential::new::<electric_potential::kilovolt>(0.0),
            wire_resolution:    quantities::LinearNumberDensity::new::<linear_number_density::per_meter>(25.0),
        }
    }
}

impl SpacecraftParameters {
    pub fn segment_length (&self) -> quantities::Length {
        // This is too hacky for my tastes, but dividing 1.0 over self.wire_resolution gave me errors
        let segment_length: f64 = 1.0 / self.wire_resolution.value;
        return quantities::Length::new::<length::meter>(segment_length);
    }

    pub fn segment_mass(&self) -> quantities::Mass {
        let segment_volume = consts::PI * self.wire_radius * self.wire_radius * self.segment_length();
        return segment_volume * self.wire_density;
    }
}



#[derive(Resource)]
#[allow(non_snake_case)]
pub struct SolarWindParameters {
    pub n_0:        quantities::VolumetricNumberDensity,    // Undisturbed solar wind electron density
    pub velocity:   quantities::Velocity, 
    pub T_e:        quantities::Energy,  // Solar wind electron temperature at 1AU
}

impl Default for SolarWindParameters {
    fn default() -> SolarWindParameters {
        SolarWindParameters {
            n_0:        quantities::VolumetricNumberDensity::new::<volumetric_number_density::per_cubic_centimeter>(7.3),
            velocity:   quantities::Velocity::new::<velocity::meter_per_second>(4.0e5), //(from google, can't find it in the paper)
            T_e:        quantities::Energy::new::<energy::electronvolt>(12.0),
        }
    }
}
