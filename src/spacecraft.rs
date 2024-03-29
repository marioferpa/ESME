use bevy::prelude::*;
use uom::si::f64 as quantities;
use uom::si::*;
use bevy::math::DVec3;
use std::f64::consts;

use crate::{ physics };
use physics::position_vector::PositionVector;

pub mod axes;
pub mod esail;
//pub mod new_esail;
pub mod body;
pub mod center_mass; 

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpacecraftParameters{..Default::default()})

            .add_systems(
                Startup, (
                    axes::spawn_axes,
                    esail::spawn_esail,
                    //new_esail::spawn_new_esail,
                    //new_esail::draw_new_esail,
                    body::spawn_cubesat,
                    center_mass::spawn_center_mass,
                )
            )
            //.add_systems(
            //    Update, (
            //        esail::click,
            //        //new_esail::update_new_esail_graphics
            //    )
            //)
        ;
    }
}

#[derive(Resource)]
pub struct SpacecraftParameters {
    pub rpm:                quantities::Frequency,  // This could be angular velocity, so I get value and direction together.
    pub rotation_axis:      DVec3,
    pub wire_length:        quantities::Length,
    pub wire_radius:        quantities::Length, 
    pub wire_density:       quantities::MassDensity,
    pub wire_potential:     quantities::ElectricPotential,
    pub wire_resolution:    quantities::LinearNumberDensity,
    pub body_size:          quantities::Length, // Will become what, a tuple of lengths?
    pub esail_origin:       PositionVector, 
}


impl Default for SpacecraftParameters {
    fn default() -> SpacecraftParameters {
        SpacecraftParameters {
            rpm:                quantities::Frequency::new::<frequency::cycle_per_minute>(0.0),
            rotation_axis:      DVec3::new(0.0, 0.0, 1.0),  // Is it correct? I think so, right hand rule
            wire_length:        quantities::Length::new::<length::meter>(1.0),
            wire_radius:        quantities::Length::new::<length::micrometer>(10.0),
            wire_density:       quantities::MassDensity::new::<mass_density::gram_per_cubic_centimeter>(2.7),
            wire_potential:     quantities::ElectricPotential::new::<electric_potential::kilovolt>(0.0),
            wire_resolution:    quantities::LinearNumberDensity::new::<linear_number_density::per_meter>(20.0),
            //wire_resolution:    quantities::LinearNumberDensity::new::<linear_number_density::per_meter>(100.0),
            body_size:          quantities::Length::new::<length::meter>(0.15),
            esail_origin:       PositionVector::new(
                                    quantities::Length::new::<length::meter>(0.15 / 2.0),
                                    quantities::Length::new::<length::meter>(0.0),
                                    quantities::Length::new::<length::meter>(0.0),
                                    ),
        }
    }
}

// Should I write a test that ensures that wire_length is a multiple of wire_resolution?

impl SpacecraftParameters {

    pub fn number_of_esail_elements(&self) -> i32 {
        let number_of_elements = self.wire_length * self.wire_resolution; 
        return number_of_elements.value as i32;
    }

    pub fn segment_length (&self) -> quantities::Length {
        // This is too hacky for my tastes, but dividing 1.0 over self.wire_resolution gave me errors
        let segment_length: f64 = 1.0 / self.wire_resolution.value;
        return quantities::Length::new::<length::meter>(segment_length);
    }

    pub fn segment_mass(&self) -> quantities::Mass {
        let segment_volume = consts::PI * self.wire_radius * self.wire_radius * self.segment_length();
        return segment_volume * self.wire_density;
    }

    /// Untested
    pub fn angular_velocity(&self) -> quantities::Frequency { 
        return self.rpm * consts::PI / 30.0;    // RPM to Radians per second 
    }
}
