use bevy::prelude::*;
use uom::si::f64 as quantities;  // Should I use f64?

#[derive(Component)]
pub struct CenterOfMass;

#[derive(Component)]
pub struct Axes;

#[derive(Component)]
pub struct SatelliteBody;

#[derive(Component, Debug)]
pub struct ESail {
    pub origin:     Entity,
    pub elements:   Vec<Entity>,
}

impl ESail {

    pub fn pixels_between_elements(
        &self,
        index: usize,
        verlet_query:   &Query<&mut VerletObject, With<SailElement>>,
        ) -> (f64, f64, f64, f64)  {

        let (current_element_x, current_element_y, current_element_z) =
            verlet_query
                .get(self.elements[index])
                .expect("Element not found")
                .current_coordinates();

        let (prev_element_x, prev_element_y, prev_element_z) =
            verlet_query
                .get(self.elements[index -1])
                .expect("Element not found")
                .current_coordinates();

        let diff_x = current_element_x - prev_element_x;
        let diff_y = current_element_y - prev_element_y;
        let diff_z = current_element_z - prev_element_z;
        let pixels_between_elements = (diff_x * diff_x + diff_y * diff_y + diff_z * diff_z).sqrt();

        return (diff_x, diff_y, diff_z, pixels_between_elements);
    }

    pub fn correct_element_coordinates(
        &self,
        index: usize,
        correction_x: f64, correction_y: f64, correction_z: f64,
        verlet_query: &mut Query<&mut VerletObject, With<SailElement>>,
        ){
        
        verlet_query
            .get(self.elements[index])
            .expect("Element not found")
            .correct_coordinates(correction_x, correction_y, correction_z);
        
    }
}

#[derive(Component, Debug)]
pub struct SailElement {
    pub is_deployed:    bool,   // Not used. Makes more sense than in VerletObject,
                                // but it's harder to access from the code.
}


#[derive(Component, Debug)]
pub struct Mass (
    pub quantities::Mass,
);

#[derive(Component)]
pub struct ElectricallyCharged {
    pub potential:  quantities::ElectricPotential,
}

// I could call this SailElement and make everything simpler
#[derive(Component, Debug, Copy, Clone)]
pub struct VerletObject {
    pub previous_x:     f64,
    pub previous_y:     f64,
    pub previous_z:     f64,
    pub current_x:      f64,
    pub current_y:      f64,
    pub current_z:      f64,
    pub is_deployed:    bool,  // This would be better in another component, SailElement maybe
}

impl VerletObject {

    pub fn current_coordinates(&self) -> (f64, f64, f64) {
        return (self.current_x, self.current_y, self.current_z);
    }

    pub fn correct_coordinates(mut self, correction_x: f64, correction_y: f64, correction_z: f64) {

        self.current_x += correction_x;
        self.current_y += correction_y;
        self.current_z += correction_z;
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
