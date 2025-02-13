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

    // https://chatgpt.com/share/67add64f-76fc-800e-8cd8-a4261dee2ed3

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

    let mut k2_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

    for (index, rk_element) in esail.rk_elements.iter().enumerate() {

        let k1_velocity = k1_vector[index as usize].1.clone();

        let intermediate_velocity = 
            rk_element.velocity.clone() + k1_velocity / 2.0;

        // Because constant force for now:
        let intermediate_acceleration = AccelerationVector::from_force(
            force.clone(), element_mass
        );

        k2_vector.push((
            PositionVector::from_velocity(
                intermediate_velocity, timestep / 2.0
            ), 
            VelocityVector::from_acceleration(
                intermediate_acceleration, timestep / 2.0
            ), 
        ));
    }

    //println!("K2: {:?}", k2_vector);
}
