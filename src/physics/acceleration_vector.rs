use uom::si::f64 as quantities;  
use bevy::math::DVec3;
use uom::si::acceleration::meter_per_second_squared;
use std::ops::{ Mul };

#[derive(Debug, Clone)]
pub struct AccelerationVector ( pub Vec<quantities::Acceleration> );

impl AccelerationVector {

    /// Creates a new AccelerationVector of capacity 3 with the requested values
    pub fn new(x: quantities::Acceleration, y: quantities::Acceleration, z: quantities::Acceleration) -> Self {
        let mut vector = Vec::with_capacity(3);
        vector.extend(vec![x, y, z]);
        return Self(vector);
    }

    /// Creates a new AccelerationVector of capacity 3 along a direction vector
    pub fn from_direction(magnitude: quantities::Acceleration, direction: DVec3) -> Self {

        let normalised_direction = direction.normalize();
        let components = normalised_direction * magnitude.get::<meter_per_second_squared>();

        let x = quantities::Acceleration::new::<meter_per_second_squared>(components.x);
        let y = quantities::Acceleration::new::<meter_per_second_squared>(components.y);
        let z = quantities::Acceleration::new::<meter_per_second_squared>(components.z);

        return Self::new(x, y, z);
    }

    // It would be better if I could just multiply a force vector times a mass and get an
    // acceleration vector
    pub fn from_force(force: super::force_vector::ForceVector, mass: quantities::Mass) -> Self {

        let acc_x = force.x() / mass;
        let acc_y = force.y() / mass;
        let acc_z = force.z() / mass;

        return Self::new(acc_x, acc_y, acc_z);
    }

    pub fn x(&self) -> quantities::Acceleration {
        self.0[0]
    }

    pub fn y(&self) -> quantities::Acceleration {
        self.0[1]
    }

    pub fn z(&self) -> quantities::Acceleration {
        self.0[2]
    }
}

impl Mul<f64> for AccelerationVector {
    type Output = Self;

    fn mul(self, value: f64) -> Self {
        let x = self.0[0] * value;
        let y = self.0[1] * value;
        let z = self.0[2] * value;
        return Self::new(x, y, z);   // Just for now
    }
}

//impl Div<f64> for AccelerationVector {
//    type Output = Self;
//
//    fn div(self, value: f64) -> Self {
//        let x = self.0[0] / value;
//        let y = self.0[1] / value;
//        let z = self.0[2] / value;
//        return Self::new(x, y, z);   // Just for now
//    }
//}
