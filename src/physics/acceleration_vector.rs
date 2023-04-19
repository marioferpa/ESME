use uom::si::f64 as quantities;  
use bevy::math::DVec3;
use uom::si::acceleration::meter_per_second_squared;
use std::ops::{ Div };

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
