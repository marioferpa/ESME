use bevy::prelude::*;
use bevy::math::DVec3;

use uom::si::*;
use uom::si::f64 as quantities;
use uom::si::force::newton;
use uom::si::length::meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;

use crate::{ physics, resources, spacecraft, time, };

use physics::acceleration_vector::AccelerationVector;
use physics::force_vector::ForceVector;
use physics::position_vector::PositionVector;
use physics::velocity_vector::VelocityVector;

// https://chatgpt.com/share/67add64f-76fc-800e-8cd8-a4261dee2ed3


pub fn runge_kutta_simulation (
    mut esail_query:        Query<&mut spacecraft::esail::ESail>,
    mut sim_params:         ResMut<resources::SimulationParameters>,
    spacecraft_parameters:  Res<spacecraft::SpacecraftParameters>,
    time:                   Res<Time>, 
) {

    let mut esail = esail_query.single_mut();

    // TODO FIXME Use real values here
    // TODO At least add the real force magnitude from the sail!!!! FIXME
    let element_mass = quantities::Mass::new::<mass::kilogram>(1.0);
    let force = ForceVector::from_direction(    // wind_force?
        quantities::Force::new::<force::newton>(0.00314),
        DVec3::new(0.0, 0.0, -1.0),
    );


    let steps = time::timestep_calculation(&time, &mut sim_params);

    let timestep = quantities::Time::new::<second>(
        time.delta_seconds() as f64 / (steps as f64 + 1.0)
    );


    for step in 0..steps {

        // Most times it's just one timestep, but I've seen two some times
        //println!("Timestep {step}");
        //println!("Elapsed time: {}", time.delta_seconds());

        let mut restoring_forces: Vec<ForceVector> = Vec::new();


        for (index, rk_object) in esail.rk_objects.iter().enumerate() {


            // First element doesn't move

            if index == 0 { 

                let zero_force =  quantities::Force::new::<newton>(0.0);
                restoring_forces.push(
                    ForceVector::new(zero_force, zero_force, zero_force)
                );

                continue 
            };



            let distance_vector = PositionVector::from_a_to_b(
                rk_object.position.clone(),
                esail.rk_objects[index-1].position.clone()
            );

            let elongation = spacecraft_parameters.segment_length() -
                distance_vector.clone().length(); 

            if index == 2 {

                println!("Elongation of second element: {:?}", elongation);
            }


            // Made-up k value!! FIXME
            let force   = quantities::Force::new::<newton>(1.0);
            let length  = quantities::Length::new::<meter>(1.0);
            let k = force / length * 10.0;


            // Damping test (made-up values as well!!) FIXME
            let force       = quantities::Force::new::<newton>(0.0001);
            let velocity    = 
                quantities::Velocity::new::<meter_per_second>(1.0);
            let c = force / velocity;


            // FIXME The elongation can be positive or negative, but as it
            // stands the damping value is always negative, so it sometimes
            // contributes to make the system stretch!?

            // At least I need to find the velocity along the line betweeen the
            // two points A along_direction() method on VelocityVector perhaps?

            // Am I doing this correctly? I want the derivative of the
            // elongation, I'm using the velocity of the particle instead?

            // I seriously can't understand how this is different with no
            // restoring force and with a restoring force of zero

            if index == 2 {

                println!(
                    "Velocity projection times c: {:?}", 
                    rk_object.velocity.project_onto(&distance_vector) * c
                );
            }

            let restoring_force = ForceVector::from_direction(
                elongation * k  // Hooke's law
                //+ rk_object.velocity.project_onto(&distance_vector) * c
                ,
                distance_vector.to_unit_vector()
            );

            //println!("Restoring force: {:?}", restoring_force);

            restoring_forces.push(restoring_force);
        }




        // K1 ------------------------------------------------------------------

        let mut k1_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        for (index, rk_object) in esail.rk_objects.iter().enumerate() {

            let velocity        = rk_object.velocity.clone();
            let acceleration    = AccelerationVector::from_force(
                //force.clone(), element_mass
                force.clone() - restoring_forces[index].clone(), element_mass
            );

            k1_vector.push((
                PositionVector::from_velocity(velocity, timestep),
                VelocityVector::from_acceleration(acceleration, timestep)
            ));
        }

        //println!("K1: {:?}", k1_vector);




        // K2 ------------------------------------------------------------------

        let mut k2_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        for (index, rk_object) in esail.rk_objects.iter().enumerate() {

            let k1_velocity = k1_vector[index].1.clone();

            let intermediate_velocity = 
                rk_object.velocity.clone() + k1_velocity / 2.0;

            // Because constant force for now:
            let intermediate_acceleration = AccelerationVector::from_force(
                //force.clone(), element_mass
                force.clone() - restoring_forces[index].clone(), element_mass
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




        // K3 ----------------------------------------------------------------------

        let mut k3_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        for (index, rk_object) in esail.rk_objects.iter().enumerate() {
            
            let k2_velocity = k2_vector[index].1.clone();

            let intermediate_velocity = 
                rk_object.velocity.clone() + k2_velocity / 2.0;

            // Because constant force for now:
            let intermediate_acceleration = AccelerationVector::from_force(
                //force.clone(), element_mass
                force.clone() - restoring_forces[index].clone(), element_mass
            );

            k3_vector.push((
                PositionVector::from_velocity(
                    intermediate_velocity, timestep / 2.0
                ), 
                VelocityVector::from_acceleration(
                    intermediate_acceleration, timestep / 2.0
                ), 
            ));
        }




        // K4 ----------------------------------------------------------------------

        let mut k4_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        for (index, rk_object) in esail.rk_objects.iter().enumerate() {
            
            let k3_velocity = k3_vector[index].1.clone();

            let intermediate_velocity =
                rk_object.velocity.clone() + k3_velocity * 2.0; // Want full step now

            // Because constant force for now:
            let intermediate_acceleration = AccelerationVector::from_force(
                //force.clone(), element_mass
                force.clone() - restoring_forces[index].clone(), element_mass
            );

            k4_vector.push((
                PositionVector::from_velocity(
                    intermediate_velocity, timestep
                ), 
                VelocityVector::from_acceleration(
                    intermediate_acceleration, timestep
                ), 
            ));
        }




        // Final step --------------------------------------------------------------

        for (index, rk_object) in esail.rk_objects.iter_mut().enumerate() {

            // Hack to avoid moving the first element (although its k's are being
            // calculated above)

            if index == 0 { continue };


            let (k1_position, k1_velocity) = k1_vector[index].clone();
            let (k2_position, k2_velocity) = k2_vector[index].clone();
            let (k3_position, k3_velocity) = k3_vector[index].clone();
            let (k4_position, k4_velocity) = k4_vector[index].clone();

            let position_increment = (
                k1_position + k2_position * 2.0 + k3_position * 2.0 + k4_position
            ) / 6.0;

            let velocity_increment = (
                k1_velocity + k2_velocity * 2.0 + k3_velocity * 2.0 + k4_velocity
            ) / 6.0;

            rk_object.position += position_increment;
            rk_object.velocity += velocity_increment;

            //println!("rk_object.position = {:?}", rk_object.position);
            //println!("rk_object.velocity = {:?}", rk_object.velocity);
        }
    }
}
