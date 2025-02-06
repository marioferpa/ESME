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

    // These three are temporary
    let element_mass = quantities::Mass::new::<mass::kilogram>(1.0);
    let force = ForceVector::from_direction(
        quantities::Force::new::<force::newton>(3.14),
        DVec3::new(0.0, 0.0, 1.0),
    );
    let timestep = quantities::Time::new::<time::second>(0.7);
    
    // Maybe I have to do this multiple times per timestep, as in verlet_sim

    let mut k1_vector: Vec<(VelocityVector, AccelerationVector)> = Vec::new();

    for rk_element in esail.rk_elements.iter() {

        k1_vector.push((
            rk_element.velocity.clone(),
            AccelerationVector::from_force(force.clone(), element_mass)
        ));
    }


    // k2 is calculated at half dt!
    let mut k2_vector: Vec<(VelocityVector, AccelerationVector)> = Vec::new();

    for (index, rk_element) in esail.rk_elements.iter().enumerate() {

        let k1_acceleration = k1_vector[index as usize].1.clone();

        let k2_velocity = VelocityVector::from_acceleration(
            k1_acceleration,
            timestep / 2.0
        );

        //k2_acceleration?
        //I'd say that this is the same as before if the springs are not there,
        //but they will be.
    }
}
