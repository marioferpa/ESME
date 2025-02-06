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
}
