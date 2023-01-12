use bevy::prelude::*;

// UOM package, for physical units
use uom::si::f32::*;    // Should I use f64?
use uom::si::length::micrometer;
use uom::si::energy::electronvolt;
use uom::lib::marker::PhantomData;
use uom::si::electric_potential::volt;
use uom::si::velocity::meter_per_second;
use uom::si::electric_permittivity::farad_per_meter; 
use uom::si::volumetric_number_density::per_cubic_centimeter;

// Maybe a constants.rs could contain these
pub const M_PROTON: Mass = Mass {dimension: PhantomData, units: PhantomData, value: 1.672e-27};
pub const Q_E: ElectricCharge = ElectricCharge {dimension: PhantomData, units: PhantomData, value: 1.602e-19};  // Is this in Coulombs, you sure?
pub const EPSILON_0: ElectricPermittivity = ElectricPermittivity {dimension: PhantomData, units: PhantomData, value: 8.854e-12};


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
    pub wire_radius:        Length,    // meters
    pub wire_resolution:    f32,    // divisions per meter
    //pub wire_potential_V:   f32,
    pub wire_potential:     ElectricPotential,
}

impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {
        SpacecraftParameters {
            rpm:                0,
            wire_length_m:      1.0,    // meters
            wire_radius_m:      0.01,   // meters
            wire_radius:        Length::new::<micrometer>(10.0),
            wire_resolution:    25.0,   // divisions per meter
            //wire_potential_V:   0.0,
            wire_potential:     ElectricPotential::new::<volt>(0.0),
        }
    }
}

#[derive(Resource)]
#[allow(non_snake_case)]
pub struct SolarWindParameters {
    pub n_0:        VolumetricNumberDensity,
    pub velocity:   Velocity, 
    pub T_e:        Energy,  // Solar wind electron temperature at 1AU

}

impl Default for SolarWindParameters {
    fn default() -> SolarWindParameters {
        SolarWindParameters {
            n_0:        VolumetricNumberDensity::new::<per_cubic_centimeter>(7.3),
            velocity:   Velocity::new::<meter_per_second>(4.0e5), //(from google, can't find it in the paper)
            T_e:        Energy::new::<electronvolt>(12.0),
        }
    }
}

