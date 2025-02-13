use uom::si::f64 as quantities;  

use std::ops::{ Add, Div, Mul };

#[derive(Debug, Clone)]
pub struct VelocityVector (
    pub Vec<quantities::Velocity>
);

impl VelocityVector {

    pub fn new (
        x: quantities::Velocity, 
        y: quantities::Velocity, 
        z: quantities::Velocity
    ) -> Self {

        let mut vector = Vec::with_capacity(3);

        vector.extend(vec![x, y, z]);

        return Self(vector);
    }

    
    pub fn from_acceleration (
        acceleration:   super::acceleration_vector::AccelerationVector,
        time:           quantities::Time,
    ) -> Self {

        let velocity_x = acceleration.x() * time;
        let velocity_y = acceleration.y() * time;
        let velocity_z = acceleration.z() * time;

        return Self::new(velocity_x, velocity_y, velocity_z);
    }

    pub fn x(&self) -> quantities::Velocity {
        return self.0[0];
    }

    pub fn y(&self) -> quantities::Velocity {
        return self.0[1];
    }

    pub fn z(&self) -> quantities::Velocity {
        return self.0[2];
    }
}

impl Add for VelocityVector {
    type Output = Self;

    fn add (self, other: Self) -> Self {

        let x = self.0[0] + other.0[0];
        let y = self.0[1] + other.0[1];
        let z = self.0[2] + other.0[2];

        return Self::new(x, y, z);
    }
}

impl Div<f64> for VelocityVector {
    type Output = Self;

    fn div (self, value: f64) -> Self {

        let x = self.0[0] / value;
        let y = self.0[1] / value;
        let z = self.0[2] / value;

        return Self::new(x, y, z);
    }
}

impl Mul<f64> for VelocityVector {
    type Output = Self;

    fn mul(self, value: f64) -> Self {
        let x = self.0[0] * value;
        let y = self.0[1] * value;
        let z = self.0[2] * value;
        return Self::new(x, y, z);
    }
}

//impl Mul<quantities::Time> for VelocityVector {
//
//    type Output = super::position_vector::PositionVector;
//
//    fn mul (
//        self, 
//        time: quantities::Time
//    ) -> super::position_vector::PositionVector {
//
//        let x = self.0[0] * time;
//        let y = self.0[1] * time;
//        let z = self.0[2] * time;
//
//        return super::position_vector::PositionVector::new(x, y, z);
//    }
//}

