use uom::si::*;
use uom::si::f64 as quantities;  

use super::position_vector::PositionVector as PositionVector;

#[derive(Debug, Clone)]
pub struct RungeKuttaObject {
    pub position:   PositionVector
    // I think I need to make a velocity vector now
}

impl RungeKuttaObject {

    pub fn default () -> Self {

        let zero =  quantities::Length::new::<length::meter>(0.0);

        return Self {
            position:   PositionVector::new(zero, zero, zero),
        }
    }
}
