use bevy::prelude::*;
use bevy::math::DVec3;  // Vec3 with f64 values
use std::ops::Sub;      // For subtracting DVec3
use std::ops::Mul;
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

    // Should it be Pixels to center?
    // Also, is this in use?
    pub fn distance_to_center(&self) -> f64 {
        return self.origin[0];  // This doesn't work for 3D, careful
    }

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
                .current_coordinates();

        let preceding_element_coords = 
            if index > 0 {
                verlet_query
                    .get(self.elements[index -1])
                    .expect("Element not found")
                    .current_coordinates()
            } else {
                self.origin
            };

        let diff_vector = current_element_coords.sub(preceding_element_coords);
        
        // Test
        let v1 = DVec3::new(1.0, 2.0, 0.0);
        let v2 = DVec3::new(3.0, 2.0, 2.0);
        //println!("Product test: {}", v1.mul(v2)); // Success
        //println!("Product test: {}", v1.mul(2.0)); // Success as well!
        //println!("Product test: {}", v1.mul(2.0 * 3.0)); // Success as well!

        return diff_vector;
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
pub struct VerletObject {   // Should these be vectors too?
    pub previous_x:     f64,
    pub previous_y:     f64,
    pub previous_z:     f64,
    pub current_x:      f64,
    pub current_y:      f64,
    pub current_z:      f64,
    pub is_deployed:    bool,  // This would be better in another component, SailElement maybe
}

impl VerletObject {

    //pub fn current_coordinates(&self) -> (f64, f64, f64) {
    //    return (self.current_x, self.current_y, self.current_z);
    //}

    pub fn current_coordinates(&self) -> DVec3 {
        return DVec3::new(self.current_x, self.current_y, self.current_z);
    }

    /// Trying Vec3 for this one
    pub fn previous_coordinates(&self) -> DVec3 {
        return DVec3::splat(0.0);
    }

    //pub fn correct_coordinates(&mut self, correction_x: f64, correction_y: f64, correction_z: f64) {    // I think this solved it omg
    //    self.current_x += correction_x;
    //    self.current_y += correction_y;
    //    self.current_z += correction_z;
    //}

    pub fn correct_current_coordinates(&mut self, correction_vector: DVec3) {    // I think this solved it omg
        self.current_x += correction_vector[0];
        self.current_y += correction_vector[1];
        self.current_z += correction_vector[2];
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
