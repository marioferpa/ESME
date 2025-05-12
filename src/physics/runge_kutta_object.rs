use uom::si::*;
use uom::si::f64 as quantities;  

use super::position_vector::PositionVector as PositionVector;
use super::velocity_vector::VelocityVector as VelocityVector;

#[derive(Debug, Clone)]
pub struct RungeKuttaObject {
    pub position:   PositionVector,
    pub velocity:   VelocityVector,
    // is_deployed, like VerletObject?
}

impl RungeKuttaObject {

    pub fn default () -> Self {

        let zero    =  quantities::Length::new::<length::meter>(0.0);
        let zero_v  =  quantities::Velocity::new::<velocity::meter_per_second>(0.0);

        return Self {
            position:   PositionVector::new(zero, zero, zero),
            velocity:   VelocityVector::new(zero_v, zero_v, zero_v)
        }
    }
}
