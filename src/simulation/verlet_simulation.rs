fn verlet_simulation(
    time:                       Res<Time>, 
    esail_query:                Query<&elements::esail::ESail>,  
    solar_wind_parameters:      Res<resources::SolarWindParameters>,
    spacecraft_parameters:      Res<elements::SpacecraftParameters>,
    mut verlet_query:           Query<&mut verlet_object::VerletObject>,
    mut simulation_parameters:  ResMut<resources::SimulationParameters>,
    ) {

    let esail = esail_query.single();

    let timesteps = timestep_calculation(&time, &mut simulation_parameters);

    for _ in 0..timesteps { 

        // VERLET INTEGRATION: Forces are calculated and applied for each esail element

        for element in esail.elements.iter() {  // Iterating over esail elements, in order.

            let mut verlet_object = verlet_query.get_mut(*element).expect("No sail element found");

            verlet_integration(&mut simulation_parameters, &mut verlet_object, &spacecraft_parameters, &solar_wind_parameters);
        }

        // CONSTRAINT LOOP. All operations in pixels, I'm pretty sure.

        for _ in 0..simulation_parameters.iterations {

            for index in 0..esail.elements.len() {  

                // Distance between element and preceding element (in METERS). 
                let distance_between_elements = esail.distance_between_elements(index, &verlet_query);    // Now is a PositionVector

                // Desired distance between elements (in METERS TOO)
                let desired_distance_between_elements = spacecraft_parameters.segment_length();

                // If difference is zero then I can skip all the rest, right? Perfect spot for an early return.

                let difference = if distance_between_elements.clone().length().value > 0.0 {
                    (desired_distance_between_elements.value - distance_between_elements.clone().length().value) / distance_between_elements.clone().length().value
                } else {
                    0.0
                };

                let correction_vector = if index > 0 {
                    distance_between_elements.mul(0.5 * difference)
                } else {
                    distance_between_elements.mul(difference)
                };

                // UPDATING POSITIONS
                
                let mut current_verlet_object = verlet_query.get_mut(esail.elements[index]).expect("No previous sail element found");

                current_verlet_object.correct_current_coordinates(correction_vector.clone());

                // Also change previous to preceding wherever needed

                if index > 0 {
                    //let preceding_sail_element   = esail.elements[index - 1];
                    let mut preceding_verlet_object = verlet_query.get_mut(esail.elements[index - 1]).expect("No previous sail element found");
                    preceding_verlet_object.correct_current_coordinates(correction_vector.mul(-1.0));
                }
            }
        }
    }
}

/// Updates the position of a verlet object
fn verlet_integration(
    simulation_parameters:  &mut ResMut<resources::SimulationParameters>,
    verlet_object:          &mut verlet_object::VerletObject,
    spacecraft_parameters:  &Res<elements::SpacecraftParameters>,
    solar_wind:             &Res<resources::SolarWindParameters>,
    ){

    // Forces per verlet (so, per segment)

    //// Centrifugal force (Along x for now, this needs to change)

    let centrifugal_force_magnitude = spacecraft_parameters.segment_mass() * verlet_object.current_coordinates.clone().length()
                                        * spacecraft_parameters.angular_velocity() * spacecraft_parameters.angular_velocity();

    let centrifugal_force_direction = DVec3::new(1.0, 0.0, 0.0);

    let centrifugal_force = force_vector::ForceVector::from_direction(centrifugal_force_magnitude, centrifugal_force_direction);

    //// Coulomb drag force
    
    let coulomb_force_magnitude= coulomb_force_per_meter(&solar_wind, &spacecraft_parameters) * spacecraft_parameters.segment_length();

    let coulomb_force = force_vector::ForceVector::from_direction(coulomb_force_magnitude, solar_wind.direction); 


    //// Total force

    let total_force = coulomb_force + centrifugal_force;    // This is a ForceVector containing uom quantities

    // Not making an acceleration uom vector just for this if the result is a position vector anyways
    let x_acceleration = total_force.x() / spacecraft_parameters.segment_mass() * simulation_parameters.timestep_s * simulation_parameters.timestep_s;
    let y_acceleration = total_force.y() / spacecraft_parameters.segment_mass() * simulation_parameters.timestep_s * simulation_parameters.timestep_s;
    let z_acceleration = total_force.z() / spacecraft_parameters.segment_mass() * simulation_parameters.timestep_s * simulation_parameters.timestep_s;

    // Wondering if these units are correct
    let position_from_acceleration = position_vector::PositionVector::new(x_acceleration, y_acceleration, z_acceleration);

    // Next position calculation (formula from here: https://www.algorithm-archive.org/contents/verlet_integration/verlet_integration.html)
    let next_coordinates = verlet_object.current_coordinates.clone().mul(2.0) - verlet_object.previous_coordinates.clone() + position_from_acceleration;

    
    // Updating verlet coordinates
    verlet_object.update_coordinates(next_coordinates);

    //println!("{}: {:?}", "Force per segment", force_per_segment);    
    //// And force per meter?
    //println!("{}: {:?}", "Total force", force_per_segment * spacecraft_parameters.wire_resolution.value * spacecraft_parameters.wire_length.value);
    //println!("-------------------------");

}
