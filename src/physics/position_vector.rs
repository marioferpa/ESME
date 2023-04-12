use uom::si::f64 as quantities;  
use uom::si::*;

use std::ops::{ Add, Sub, Mul };

#[derive(Debug, Clone)]
pub struct PositionVector ( pub Vec<quantities::Length> );

impl PositionVector {

    /// Return a new PositionVector of capacity 3 with the requested values
    pub fn new(x: quantities::Length, y: quantities::Length, z: quantities::Length) -> Self {
        let mut vector = Vec::with_capacity(3);
        vector.extend(vec![x, y, z]);
        return Self(vector);
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        return PositionVector( Vec::new() );    // Make it capacity 3
    }

    /// Returns the length of the PositionVector
    pub fn length(self) -> quantities::Length {
        
        let x = self.0[0] * self.0[0];
        let y = self.0[1] * self.0[1];
        let z = self.0[2] * self.0[2];

        let length = (x.value + y.value + z.value).sqrt();

        return quantities::Length::new::<length::meter>(length);    // Finish this
    }

    pub fn x(&self) -> quantities::Length {
        return self.0[0];
    }

    /// Returns a new position vector with zero values
    pub fn zero() -> Self {
        let zero = quantities::Length::new::<length::meter>(0.0);
        let mut vector = Vec::with_capacity(3);
        vector.extend(vec![zero, zero, zero]);
        return Self(vector);
    }

    // Returns the opposite vector
    // Don't know why it fails
    //pub fn negative() -> Self {
    //    return Self.mul(-1.0);
    //}
}


impl Add for PositionVector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.0[0] + other.0[0];
        let y = self.0[1] + other.0[1];
        let z = self.0[2] + other.0[2];
        return Self::new(x, y, z);
    }
}

impl Sub for PositionVector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.0[0] - other.0[0];
        let y = self.0[1] - other.0[1];
        let z = self.0[2] - other.0[2];
        return Self::new(x, y, z);
    }
}

impl Mul<f64> for PositionVector {
    type Output = Self;

    fn mul(self, value: f64) -> Self {
        let x = self.0[0] * value;
        let y = self.0[1] * value;
        let z = self.0[2] * value;
        return Self::new(x, y, z);   // Just for now
    }
}
