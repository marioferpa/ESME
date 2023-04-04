use uom::si::f64 as quantities;  
use bevy::math::DVec3;
use uom::si::force::newton;

use std::ops::{ Add };

#[derive(Debug, Clone)]
pub struct ForceVector ( pub Vec<quantities::Force> );

impl ForceVector {

    /// Creates a new ForceVector of capacity 3 with the requested values
    pub fn new(x: quantities::Force, y: quantities::Force, z: quantities::Force) -> Self {
        let mut vector = Vec::with_capacity(3);
        //vector.push(x);
        //vector.push(y);
        //vector.push(z);
        vector.extend(vec![x, y, z]);
        return Self(vector);
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
