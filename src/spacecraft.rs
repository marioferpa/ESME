use bevy::prelude::*;
use uom::si::f64 as quantities;
use uom::si::*;
use bevy::math::DVec3;
use std::f64::consts;

use crate::{ physics, settings };
use physics::position_vector::PositionVector;

pub mod esail;
pub mod body;
pub mod center_mass; 

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpacecraftParameters{..Default::default()})

            .add_systems(
                PreStartup, (
                    esail::spawn_esail,
                )
            )
            // TODO At least some of these should go to graphics.rs
            .add_systems(
                Startup, (
                    body::spawn_cubesat,
                    center_mass::spawn_center_mass,
                )
            )
        ;
    }
}

#[derive(Resource)]
pub struct SpacecraftParameters {
    // This could be angular velocity, so I get value and direction together.
    pub rpm:                quantities::Frequency,
    pub rotation_axis:      DVec3,
    pub tether_length:      quantities::Length,
    pub tether_radius:      quantities::Length, 
    pub tether_density:     quantities::MassDensity,
    pub tether_potential:   quantities::ElectricPotential,
    pub tether_resolution:  quantities::LinearNumberDensity,
    pub body_size:          quantities::Length,
    pub tether_origin:      PositionVector, 
}


impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {

        let default_rpm =    
            quantities::Frequency::new::<frequency::cycle_per_minute>(0.0);

        let default_rotation_axis =      
            DVec3::new(0.0, 0.0, 1.0);

        let default_tether_length =        
            quantities::Length::new::<length::meter>
                (settings::TETHER_LENGTH_METERS);

        let default_tether_radius =
            quantities::Length::new::<length::micrometer>
                (settings::TETHER_RADIUS_MICROMETERS);

        let default_tether_density =
            quantities::MassDensity::new::
                <mass_density::gram_per_cubic_centimeter>(2.7);

        let default_tether_potential = 
            quantities::ElectricPotential::new::
                <electric_potential::kilovolt>(0.0);

        let default_tether_potential =     
            quantities::ElectricPotential::new::
                <electric_potential::kilovolt>(0.0);

        let default_tether_resolution = 
            quantities::LinearNumberDensity::new::
                <linear_number_density::per_meter>
                    (settings::TETHER_POINTS_PER_METER);

        let body_size = quantities::Length::new::<length::meter>(0.15);

        // TODO Make it relative to body size
        let tether_origin = PositionVector::new(
            quantities::Length::new::<length::meter>(0.15 / 2.0),
            quantities::Length::new::<length::meter>(0.0),
            quantities::Length::new::<length::meter>(0.0),
        );

        SpacecraftParameters {
            rpm:                default_rpm,
            rotation_axis:      default_rotation_axis,
            tether_length:      default_tether_length,
            tether_radius:      default_tether_radius,
            tether_density:     default_tether_density,
            tether_potential:   default_tether_potential,
            tether_resolution:  default_tether_resolution,
            body_size,
            tether_origin
        }
    }
}

// Should I write a test that ensures that wire_length is a multiple of wire_resolution?

impl SpacecraftParameters {

    pub fn number_of_esail_elements(&self) -> i32 {
        let number_of_elements = self.tether_length * self.tether_resolution; 
        return number_of_elements.value as i32;
    }

    pub fn segment_length (&self) -> quantities::Length {
        let segment_length: f64 = 1.0 / self.tether_resolution.value;
        return quantities::Length::new::<length::meter>(segment_length);
    }

    pub fn segment_mass(&self) -> quantities::Mass {
        let segment_volume = consts::PI * self.tether_radius * self.tether_radius * self.segment_length();
        return segment_volume * self.tether_density;
    }

    /// Untested
    pub fn angular_velocity(&self) -> quantities::Frequency { 
        return self.rpm * consts::PI / 30.0;    // RPM to Radians per second 
    }
}
