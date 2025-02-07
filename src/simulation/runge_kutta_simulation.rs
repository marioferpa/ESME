use bevy::prelude::*;
use bevy::math::DVec3;

use uom::si::*;
use uom::si::f64 as quantities;

use crate::{ physics, spacecraft, };

use physics::acceleration_vector::AccelerationVector as AccelerationVector;
use physics::force_vector::ForceVector as ForceVector;
use physics::position_vector::PositionVector as PositionVector;
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
    let timestep = quantities::Time::new::<time::second>(0.8);

    // TODO Maybe I have to do this multiple times per timestep, as in verlet_sim




    // K1 ----------------------------------------------------------------------

    let mut k1_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

    for rk_element in esail.rk_elements.iter() {

        let velocity        = rk_element.velocity.clone();
        let acceleration    = AccelerationVector::from_force(
            force.clone(), element_mass
        );

        k1_vector.push((
            PositionVector::from_velocity(velocity, timestep),
            VelocityVector::from_acceleration(acceleration, timestep)
        ));
    }

    //println!("K1: {:?}", k1_vector);




    // K2 ----------------------------------------------------------------------

    //let mut k2_vector: Vec<(VelocityVector, AccelerationVector)> = Vec::new();

    //// I may need an intermediate state here, and to operate on it

    //for (index, rk_element) in esail.rk_elements.iter().enumerate() {


    //    let k1_velocity     = k1_vector[index as usize].0.clone();
    //    let k1_acceleration = k1_vector[index as usize].1.clone();

    //    let k2_velocity = k1_velocity + VelocityVector::from_acceleration(
    //        k1_acceleration.clone(),
    //        timestep / 2.0
    //    );

    //    k2_vector.push((
    //        k2_velocity, k1_acceleration
    //    ));
    //}

    //println!("K2: {:?}", k2_vector);
}
