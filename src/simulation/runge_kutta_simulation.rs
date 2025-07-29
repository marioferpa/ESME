use bevy::prelude::*;

use uom::si::*;
use uom::si::f64 as quantities;
use uom::si::force::newton;
use uom::si::length::meter;
use uom::si::time::second;

use crate::{ physics, resources, solar_wind, spacecraft, time, };

use physics::acceleration_vector::AccelerationVector;
use physics::force_vector::ForceVector;
use physics::position_vector::PositionVector;
use physics::velocity_vector::VelocityVector;


// Problem now is (without damping): The elements that are far from the pivot
// point take a long time to start moving, so longer space to cover, they reach
// a high velocity and then they have a lot of inertia, so they overshoot.

// TODO Why's the last element moving so fast, I don't get it

pub fn runge_kutta_simulation (
    mut esail_query:        Query<&mut spacecraft::esail::ESail>,
    mut sim_params:         ResMut<resources::SimulationParameters>,
    spacecraft_parameters:  Res<spacecraft::SpacecraftParameters>,
    solar_wind:             Res<solar_wind::SolarWind>,
    time:                   Res<Time>, 
) {

    let mut esail = esail_query.single_mut();

    let coulomb_force_magnitude = 
        physics::coulomb_force_per_meter(
            &solar_wind, &spacecraft_parameters
        ) * spacecraft_parameters.segment_length();


    // Fictional, update TODO
    //let element_mass = quantities::Mass::new::<mass::kilogram>(0.01);

    let element_mass = spacecraft_parameters.segment_mass();
    //println!("Segment mass: {:?}", element_mass);

    let wind_force = ForceVector::from_direction(
        coulomb_force_magnitude, 
        solar_wind.direction
    ); 

    //println!("Wind force: {:?}", wind_force);


    let steps = time::timestep_calculation(&time, &mut sim_params);

    let timestep = quantities::Time::new::<second>(
        time.delta_seconds() as f64 / (steps as f64 + 1.0)
    );


    for _step in 0..steps {


        // K1 ------------------------------------------------------------------

        // Apart from the restoring_forces function, I'd say K1 is pretty
        // straigh-forward

        let (rk_positions, rk_velocities): 
            (Vec<PositionVector>, Vec<VelocityVector>) = esail.rk_objects
                .iter()
                .map(|obj| (obj.position.clone(), obj.velocity.clone()))
                .unzip();


        let restoring_forces = calculate_restoring_forces(
            rk_positions.clone(), rk_velocities.clone(), &spacecraft_parameters
        );


        let mut k1_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        for (index, rk_object) in esail.rk_objects.iter().enumerate() {

            let velocity        = rk_object.velocity.clone();

            let acceleration    = AccelerationVector::from_force(
                wind_force.clone() - restoring_forces[index].clone(), element_mass
            );

            k1_vector.push((
                PositionVector::from_velocity(velocity, timestep),
                VelocityVector::from_acceleration(acceleration, timestep)
            ));
        }




        // K2 ------------------------------------------------------------------



        let mut k2_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        // FIXME k1_velocities not used! Is that a mistake?

        let (k1_positions, _k1_velocities): 
            (Vec<PositionVector>, Vec<VelocityVector>) = k1_vector
                .iter()
                .map(|vec| (vec.0.clone(), vec.1.clone()))
                .unzip();


        let updated_positions: Vec<PositionVector> = rk_positions
            .iter()
            .zip(k1_positions.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();


        let restoring_forces = calculate_restoring_forces(
            updated_positions, rk_velocities.clone(), &spacecraft_parameters
        );



        for (index, rk_object) in esail.rk_objects.iter().enumerate() {

            let k1_velocity = k1_vector[index].1.clone();

            let intermediate_velocity = 
                rk_object.velocity.clone() + k1_velocity / 2.0;

            let intermediate_acceleration = 
                AccelerationVector::from_force(
                    wind_force.clone() - restoring_forces[index].clone(), 
                    element_mass
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




        // K3 ------------------------------------------------------------------


        // FIXME k2_velocities not used! Is that a mistake?

        let (k2_positions, _k2_velocities): 
            (Vec<PositionVector>, Vec<VelocityVector>) = k2_vector
                .iter()
                .map(|vec| (vec.0.clone(), vec.1.clone()))
                .unzip();
        
        
        let updated_positions: Vec<PositionVector> = rk_positions
            .iter()
            .zip(k2_positions.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();

        let restoring_forces = calculate_restoring_forces(
            //updated_positions, &spacecraft_parameters
            updated_positions, rk_velocities.clone(), &spacecraft_parameters
        );


        let mut k3_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        for (index, rk_object) in esail.rk_objects.iter().enumerate() {
            
            let k2_velocity = k2_vector[index].1.clone();

            let intermediate_velocity = 
                rk_object.velocity.clone() + k2_velocity / 2.0;

            // Because constant force for now:
            let intermediate_acceleration = AccelerationVector::from_force(
                //wind_force.clone(), element_mass
                wind_force.clone() - restoring_forces[index].clone(), element_mass
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

        // FIXME k3_velocities not used! Is that a mistake?

        let (k3_positions, _k3_velocities): 
            (Vec<PositionVector>, Vec<VelocityVector>) = k3_vector
                .iter()
                .map(|vec| (vec.0.clone(), vec.1.clone()))
                .unzip();

        let updated_positions: Vec<PositionVector> = rk_positions
            .iter()
            .zip(k3_positions.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();

        let restoring_forces = calculate_restoring_forces(
            //updated_positions, &spacecraft_parameters
            updated_positions, rk_velocities.clone(), &spacecraft_parameters
        );


        let mut k4_vector: Vec<(PositionVector, VelocityVector)> = Vec::new();

        for (index, rk_object) in esail.rk_objects.iter().enumerate() {
            
            let k3_velocity = k3_vector[index].1.clone();

            let intermediate_velocity =
                rk_object.velocity.clone() + k3_velocity * 2.0; // Want full step now

            // Because constant force for now:
            let intermediate_acceleration = AccelerationVector::from_force(
                //wind_force.clone(), element_mass
                wind_force.clone() - restoring_forces[index].clone(), element_mass
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

            // First element is fixed
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



fn calculate_restoring_forces (
    rk_objects_positions:   Vec<PositionVector>,
    _rk_objects_velocities:  Vec<VelocityVector>,    // Not used! A mistake?
    spacecraft_parameters:  &Res<spacecraft::SpacecraftParameters>,
) -> Vec<ForceVector> {

    // I have the Vec<VelocityVector> now, but I guess I have to update it on
    // every step as well? TODO Try that before deactivating it again
    
    let mut restoring_forces: Vec<ForceVector> = Vec::new();

    for (index, rk_object_position) in rk_objects_positions.iter().enumerate() {

        if index == 0 { 

            // First element doesn't move

            restoring_forces.push(ForceVector::zero());

            continue 
        };


        let distance_vector = PositionVector::from_a_to_b(
            rk_object_position.clone(),
            rk_objects_positions[index-1].clone()
        );

        //let elongation = spacecraft_parameters.segment_length() -
        //    distance_vector.clone().length(); 

        // Trying this advice from Pekka
        let elongation = (spacecraft_parameters.segment_length() - 
            distance_vector.clone().length()).min(quantities::Length::new::<meter>(0.0));

        //println!("Elongation: {:?}", elongation);

        // Made-up k value!! FIXME
        let force   = quantities::Force::new::<newton>(1.0);
        let length  = quantities::Length::new::<meter>(1.0);
        let k = force / length * 0.15;


        // Damping test (made-up values as well!!)
        // Keeps exploding...

        //let force       = quantities::Force::new::<newton>(0.0001);
        //let velocity    = 
        //    quantities::Velocity::new::<meter_per_second>(1.0);
        //let c = force / velocity;

        let restoring_force = ForceVector::from_direction(
            elongation * k  // Hooke's law
            //+ rk_objects_velocities[index].project_onto(&distance_vector) * c
            ,
            distance_vector.to_unit_vector()
        );

        //println!("Restoring force: {:?}", restoring_force);

        restoring_forces.push(restoring_force);
    }

    return restoring_forces
}
