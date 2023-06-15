use uom::si::f64 as quantities;  
use uom::si::*;
use bevy::math::DVec3;
use uom::si::force::newton;

use std::ops::{ Add, Div };

#[derive(Debug, Clone)]
pub struct ForceVector ( pub Vec<quantities::Force> );

// Can I / should I implement a division of force over mass and getting acceleration?

impl ForceVector {

    /// Creates a new ForceVector of capacity 3 with the requested values
    pub fn new(x: quantities::Force, y: quantities::Force, z: quantities::Force) -> Self {
        let mut vector = Vec::with_capacity(3);
        vector.extend(vec![x, y, z]);
        return Self(vector);
    }

    /// Creates a new ForceVector with all zeros.
    pub fn empty() -> Self {
        return Self( Vec::with_capacity(3) );
    }

    /// Creates a new ForceVector of capacity 3 along a direction vector
    pub fn from_direction(magnitude: quantities::Force, direction: DVec3) -> Self {

        let normalised_direction = direction.normalize();
        let components = normalised_direction * magnitude.get::<newton>();

        let x = quantities::Force::new::<newton>(components.x);
        let y = quantities::Force::new::<newton>(components.y);
        let z = quantities::Force::new::<newton>(components.z);

        return Self::new(x, y, z);
    }

    pub fn x(&self) -> quantities::Force {
        self.0[0]
    }

    pub fn y(&self) -> quantities::Force {
        self.0[1]
    }

    pub fn z(&self) -> quantities::Force {
        self.0[2]
    }

    /// Returns the length of the ForceVector
    // Should be called "magnitude" maybe
    pub fn length(self) -> quantities::Length {
        
        let x = self.0[0] * self.0[0];
        let y = self.0[1] * self.0[1];
        let z = self.0[2] * self.0[2];

        let length = (x.value + y.value + z.value).sqrt();

        return quantities::Length::new::<length::meter>(length);    // Finish this
    }

    //pub fn direction(&self) -> DVec3 {
    //}

}

impl Add for ForceVector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.0[0] + other.0[0];
        let y = self.0[1] + other.0[1];
        let z = self.0[2] + other.0[2];
        return Self::new(x, y, z);
    }
}

impl Div<f64> for ForceVector {
    type Output = Self;

    fn div(self, value: f64) -> Self {
        let x = self.0[0] / value;
        let y = self.0[1] / value;
        let z = self.0[2] / value;
        return Self::new(x, y, z);   // Just for now
    }
}
