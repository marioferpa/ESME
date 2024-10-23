use uom::si::f64 as quantities;  
use uom::si::*;
use uom::si::length::meter;

use std::ops::{ Add, Sub, Mul };

pub fn dot_product (
    first: &PositionVector, 
    second: &PositionVector
) -> f64 {
    
    let result_x = first.x().get::<meter>() * second.x().get::<meter>();
    let result_y = first.y().get::<meter>() * second.y().get::<meter>();
    let result_z = first.z().get::<meter>() * second.z().get::<meter>();

    return result_x + result_y + result_z;
}


pub fn angle_between (
    first: &PositionVector, 
    second: &PositionVector
) -> quantities::Angle {

    let dot_product = dot_product(&first, &second);
    let magnitudes_product = 
        first.clone().length().get::<meter>() * 
        second.clone().length().get::<meter>();

    let cos_theta = dot_product / magnitudes_product;

    //let angle_value = cos_theta.acos() * 180.0 / PI;
    let angle_value = cos_theta.acos();

    return quantities::Angle::new::<angle::radian>(angle_value);
}



#[derive(Debug, Clone)]
pub struct PositionVector ( pub Vec<quantities::Length> );

impl PositionVector {

    //TODO I think the angle should be radians directly
    pub fn rotate_z (&mut self, angle: f32) -> Self {

        let x = self.0[0];
        let y = self.0[1];
        let z = self.0[2]; // This stays the same

        // Convert the angle to a Quantity type (radians assumed)
        let cos_angle = 
            quantities::Angle::new::<uom::si::angle::radian>(
                angle.cos().into()
            );

        let sin_angle = 
            quantities::Angle::new::<uom::si::angle::radian>(
                angle.sin().into()
            );

        let new_x = x * cos_angle - y * sin_angle;
        let new_y = x * sin_angle + y * cos_angle;

        return Self::new(new_x, new_y, z);
    }

    /// Return a new PositionVector of capacity 3 with the requested values
    pub fn new (
        x: quantities::Length, 
        y: quantities::Length, 
        z: quantities::Length
    ) -> Self {

        let mut vector = Vec::with_capacity(3);

        vector.extend(vec![x, y, z]);

        return Self(vector);
    }



    #[allow(dead_code)]
    pub fn empty () -> Self {
        return PositionVector( Vec::new() );    // Make it capacity 3
    }

    // Untested ?
    pub fn from_a_to_b (
        point_a: Self,
        point_b: Self,
    ) -> Self {

        let x = point_b.x() - point_a.x();
        let y = point_b.y() - point_a.y();
        let z = point_b.z() - point_a.z();

        return Self::new(x, y, z);
    }

    pub fn from_acceleration (
        acceleration: super::acceleration_vector::AccelerationVector, 
        time: quantities::Time
    ) -> Self {

        let pos_x = acceleration.x() * time * time;
        let pos_y = acceleration.y() * time * time;
        let pos_z = acceleration.z() * time * time;
    
        return Self::new(pos_x, pos_y, pos_z);
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

    pub fn y(&self) -> quantities::Length {
        return self.0[1];
    }

    pub fn z(&self) -> quantities::Length {
        return self.0[2];
    }

    //pub fn zero() -> Self {
    //    let zero = quantities::Length::new::<length::meter>(0.0);
    //    let mut vector = Vec::with_capacity(3);
    //    vector.extend(vec![zero, zero, zero]);
    //    return Self(vector);
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
