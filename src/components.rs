use bevy::prelude::*;
use bevy::math::DVec3;  // Vec3 with f64 values
use std::ops::Sub;      // For subtracting DVec3
use std::ops::Add;      // For adding DVec3
use uom::si::f64 as quantities;  
use uom::si::electric_potential::volt;

#[derive(Component)]
pub struct CenterOfMass;

#[derive(Component)]
pub struct Axes;

#[derive(Component)]
pub struct SatelliteBody;

#[derive(Component, Debug)]
pub struct ESail {
    pub origin:     DVec3,
    pub elements:   Vec<Entity>,
}

impl ESail {

    // Modify this so that if index = 0 it calculates distance to origin instead
    pub fn pixels_between_elements(
        &self,
        index: usize,
        verlet_query:   &Query<&mut VerletObject>,
        ) -> DVec3 {

        let current_element_coords =
            verlet_query
                .get(self.elements[index])
                .expect("Element not found")
                .current_coordinates;

        let preceding_element_coords = 
            if index > 0 {
                verlet_query
                    .get(self.elements[index -1])
                    .expect("Element not found")
                    .current_coordinates
            } else {
                self.origin
            };

        return current_element_coords.sub(preceding_element_coords);
    }
}

#[derive(Component, Debug)]
pub struct Mass (
    pub quantities::Mass,
);

#[derive(Component)]
pub struct ElectricallyCharged {
    pub potential:  quantities::ElectricPotential,
}

impl Default for ElectricallyCharged {
    fn default() -> Self {
        ElectricallyCharged {
            potential: quantities::ElectricPotential::new::<volt>(0.0),
        }
    }
}
 
// I could rename this to SailElement and make everything simpler
#[derive(Component, Debug, Copy, Clone)]
pub struct VerletObject { 
    pub previous_coordinates:   DVec3,
    pub current_coordinates:    DVec3,
    pub is_deployed:            bool,  // This would be better in another component, SailElement maybe
}

impl VerletObject {

    pub fn correct_current_coordinates(&mut self, correction_vector: DVec3) {    // I think this solved it omg
        self.current_coordinates = self.current_coordinates.add(correction_vector); //Check if add works as you think)
    }
}

/// Tags an entity as capable of panning and orbiting. Taken from Bevy cheatbook
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}
