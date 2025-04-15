use uom::si::f64 as quantities;  
use uom::si::velocity;

use std::ops::{ Add, AddAssign, Div, Mul };

#[derive(Debug, Clone)]
pub struct VelocityVector (
    pub Vec<quantities::Velocity>
);

impl VelocityVector {

    //pub fn empty () -> Self {

    //    return Self( Vec::with_capacity(3) );
    //}

    // Test
    pub fn modulus_m_per_s (self) -> quantities::Velocity {
        
        let x = self.x() * self.x();
        let y = self.y() * self.y();
        let z = self.z() * self.z();

        let velocity = (x.value + y.value + z.value).sqrt();

        quantities::Velocity::new::<velocity::meter_per_second>(velocity)
    }

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

impl AddAssign for VelocityVector {

    fn add_assign (&mut self, other: Self) {

        for (a, b) in self.0.iter_mut().zip(other.0) {
            *a += b;
        }
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
