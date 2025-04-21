use bevy::math::DVec3;
use std::ops::{ Add, AddAssign, Div, Mul };
use uom::si::f64 as quantities;  
use uom::si::velocity;

use super::position_vector::PositionVector;

#[derive(Debug, Clone)]
pub struct VelocityVector (
    pub Vec<quantities::Velocity>
);

impl VelocityVector {

    // Test
    pub fn project_onto(
        &self, 
        direction: &PositionVector
    ) -> quantities::Velocity {

        let velocity_unit = self.to_unit_vector();
        let direction_unit = direction.to_unit_vector();

        //let angle_radians = velocity_unit.angle_between(direction_unit);
        let angle_radians = direction_unit.angle_between(velocity_unit);

        // Now modulus is self.modulus_m_per_s times cosine of angle, and vector
        // would be that times direction

        let projected_v_scalar = self.clone().modulus() * angle_radians.cos();

        let projected_velocity = VelocityVector::from_direction(
            projected_v_scalar,
            direction_unit,
        );

        // Everything breaks, and projected velocity is NaN. I don't know if
        // it's NaN because it breaks or if it breaks because it's NaN
        //
        // It's NaN as a consequence, because if I don't use it in the restoring
        // force the projected velocity seems alright (in the correct order of
        // magnitude at least)

        println!("");
        println!("Velocity: {:?}", self.0);
        println!("Projected velocity: {:?}", projected_velocity);

        return projected_velocity.modulus();
    }


    // Test
    pub fn to_unit_vector (&self) -> DVec3 {

        let velocities: [f64; 3] = [
            self.x().get::<velocity::meter_per_second>(),
            self.y().get::<velocity::meter_per_second>(),
            self.z().get::<velocity::meter_per_second>(),
        ];

        let vec = DVec3::from(velocities);

        return vec.normalize()
    }

    pub fn from_direction (
        velocity:   quantities::Velocity, 
        direction:  DVec3
    ) -> Self {

        let normalised_direction = direction.normalize();
        let components = normalised_direction * velocity.get::<velocity::meter_per_second>();

        let x = quantities::Velocity::new::<velocity::meter_per_second>(components.x);
        let y = quantities::Velocity::new::<velocity::meter_per_second>(components.y);
        let z = quantities::Velocity::new::<velocity::meter_per_second>(components.z);

        return Self::new(x, y, z);
    }


    pub fn modulus (self) -> quantities::Velocity {
        
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
