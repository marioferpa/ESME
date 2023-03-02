use bevy::prelude::*;
use bevy::math::DVec3;  // Vec3 with f64 values
use std::ops::Sub;      // For subtracting DVec3
use std::ops::Add;      // For adding DVec3
use uom::si::f64 as quantities;  
use uom::si::electric_potential::volt;
use uom::si::*;

// TEST! If it works move it somewhere better
#[derive(Debug)]
pub struct PositionVector ( Vec<quantities::Length> );

impl PositionVector {

    pub fn empty() -> Self {
        return PositionVector( Vec::new() );
    }

    pub fn new(x: quantities::Length, y: quantities::Length, z: quantities::Length) -> Self {
        return PositionVector( vec![x, y, z] );
    }
}

impl Add for PositionVector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.0[0] + other.0[0];
        let y = self.0[1] + other.0[1];
        let z = self.0[2] + other.0[2];
        return Self(vec![x, y, z]); 
    }
}

impl Sub for PositionVector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.0[0] - other.0[0];
        let y = self.0[1] - other.0[1];
        let z = self.0[2] - other.0[2];
        return Self(vec![x, y, z]); 
    }
}

///////////////////////////////////////////////////////

#[derive(Component)]
pub struct CenterOfMass;

#[derive(Component)]
pub struct Axes;

#[derive(Component)]
pub struct SatelliteBody;

#[derive(Component, Debug)]
pub struct ESail {
    pub origin:     DVec3,
    // NEW!
    //pub origin_new:     Vec<quantities::Length>,
    pub origin_new: PositionVector,
    pub elements:   Vec<Entity>,
}


impl ESail {

    // Test
    pub fn distance_between_elements(&self, index: usize, verlet_query: &Query<&mut VerletObject>) -> PositionVector {

        let element_position = &verlet_query.get(self.elements[index]).expect("").current_coordinates_new;

        let preceding_element_position = 
            if index > 0 {
                &verlet_query.get(self.elements[index-1]).expect("").current_coordinates_new
            } else {
                &self.origin_new
            };

        //return element_position - preceding_element_position; // This won't work
        return PositionVector::empty();
    }

    pub fn pixels_between_elements( // Are they actually pixels? You're mixing up stuff, dude
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

        // TEST, remove later
        //let quantity1 = quantities::Length::new::<length::meter>(1.1);
        //let quantity2 = quantities::Length::new::<length::meter>(2.2);
        //let quantity3 = quantities::Length::new::<length::meter>(3.3);
        //let vector1 = PositionVector(vec![quantity1, quantity2, quantity3]);
        //let vector2 = PositionVector(vec![quantity1, quantity2, quantity3]);
        ////println!("{:?}", vector1 + vector2);
        //println!("{:?}", vector1 - vector2);

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
 
#[derive(Component)]
pub struct Position (
    pub Vec<quantities::Length>,
);

// pub struct rotation

// I could rename this to SailElement and make everything simpler
//#[derive(Component, Debug, Copy, Clone)]
#[derive(Component, Debug)]
pub struct VerletObject { 
    pub previous_coordinates:   DVec3,  // Are these meters or what? They should!
    pub current_coordinates:    DVec3,
    //pub previous_coordinates_new:   Vec<quantities::Length>,
    //pub current_coordinates_new:    Vec<quantities::Length>,
    pub previous_coordinates_new:   PositionVector,
    pub current_coordinates_new:    PositionVector,
}

impl VerletObject {

    pub fn correct_current_coordinates(&mut self, correction_vector: DVec3) {    // I think this solved it omg
        self.current_coordinates = self.current_coordinates.add(correction_vector); //Check if add works as you think)
    }

    /// Previous position if forgotten, current coordinates become previous coordinates, and next coordinates become current coordinates.
    pub fn update_coordinates(&mut self, next_coordinates: DVec3) {
        self.previous_coordinates = self.current_coordinates;
        self.current_coordinates  = next_coordinates;
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
