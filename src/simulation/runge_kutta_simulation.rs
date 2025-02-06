use bevy::prelude::*;
use bevy::math::DVec3;

use uom::si::*;
use uom::si::f64 as quantities;

use crate::{ physics, spacecraft, };

use physics::acceleration_vector::AccelerationVector as AccelerationVector;
use physics::force_vector::ForceVector as ForceVector;
use physics::velocity_vector::VelocityVector as VelocityVector;

pub fn runge_kutta_simulation (
    mut esail_query:    Query<&mut spacecraft::esail::ESail>,
) {

    let mut esail = esail_query.single_mut();

    // These two are temporary
    let element_mass = quantities::Mass::new::<mass::kilogram>(1.0);
    let force = ForceVector::from_direction(
        quantities::Force::new::<force::newton>(3.14),
        DVec3::new(0.0, 0.0, 1.0),
    );
    
    // Maybe I have to do this multiple times per timestep, as in verlet_sim

    let mut k1s_vector: Vec<(VelocityVector, AccelerationVector)> = Vec::new();

    for rk_element in esail.rk_elements.iter() {

        k1s_vector.push((
            rk_element.velocity.clone(),
            AccelerationVector::from_force(force.clone(), element_mass)
        ));
    }
}
