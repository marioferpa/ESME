use bevy::prelude::*;
use bevy::math::DVec3;

use uom::si::*;
use uom::si::f64 as quantities;

#[derive(Resource)]
#[allow(non_snake_case)]
pub struct SolarWind {
    pub n_0:        quantities::VolumetricNumberDensity,    // Undisturbed solar wind electron density
    pub velocity:   quantities::Velocity, 
    pub direction:  DVec3,
    pub T_e:        quantities::Energy,                     // Solar wind electron temperature
}

impl Default for SolarWind {

    fn default() -> SolarWind {
        SolarWind {
            n_0:        quantities::VolumetricNumberDensity::new::<volumetric_number_density::per_cubic_centimeter>(7.3),
            velocity:   quantities::Velocity::new::<velocity::meter_per_second>(4.0e5), //(from google, can't find it in the paper)
            direction:  DVec3::new(0.0, 0.0, -1.0), // It should be a unit vector, right?
            T_e:        quantities::Energy::new::<energy::electronvolt>(12.0),          // Solar wind electron temperature at 1AU
        }
    }
}
