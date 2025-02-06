use uom::si::f64 as quantities;  

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
}
