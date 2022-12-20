// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// Problem, maybe: The simulation seems to be idle for the two first frames

use std::f32::consts;

use bevy::prelude::*;
use crate::{ components, resources };

pub const PHYSICS_TIMESTEP: f32 = 1.0/60.0; // seconds
pub const ITERATIONS:       i32 = 11;    

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(resources::SimulationParameters{timestep: PHYSICS_TIMESTEP, iterations: ITERATIONS, ..Default::default()})
            .add_system(verlet_simulation)
            .add_system(update_transform_verlets) 
            .add_system(update_center_of_mass)
            ;
    }
}

/// Calculates how many timesteps should happen in the current frame, considering any potential unspent time from the previous frame.
fn timestep_calculation(
    time: &Res<Time>,
    sim_params: &mut ResMut<resources::SimulationParameters>,
    ) -> i32 {

    // Time since last update plus leftover time from previous frame
    let elapsed_time = time.delta_seconds() + sim_params.leftover_time; 

    // Number of timesteps that should be performed during this frame
    let timesteps = (elapsed_time / sim_params.timestep).floor() as i32; 

    // Calculating and storing leftover time, to be used in the next frame
    let leftover_time = elapsed_time - timesteps as f32 * sim_params.timestep;
    sim_params.leftover_time = leftover_time;

    return timesteps;
}

/// Updates the position of a verlet object
fn verlet_integration(
    verlet_object:  &mut components::VerletObject,
    // Also the transform, the voltage... Should I pass them all together somehow?
    sim_params:     &mut ResMut<resources::SimulationParameters>,
    craft_params:   &Res<resources::SpacecraftParameters>,
    ){

    // VELOCITIES

    let current_position_x  = verlet_object.current_x;
    let current_position_y  = verlet_object.current_y;

    let previous_position_x = verlet_object.previous_x;
    let previous_position_y = verlet_object.previous_y;

    let velocity_x = current_position_x - previous_position_x;
    let velocity_y = current_position_y - previous_position_y;


    // FORCES

    // X AXIS: Centrifugal force

    // This should be distance to the center of mass
    let distance_to_center = (current_position_x * current_position_x + current_position_y * current_position_y).sqrt();

    let angular_velocity = craft_params.rpm as f32 * consts::PI / 30.0;

    let acceleration_x = distance_to_center * angular_velocity * angular_velocity;

    let next_position_x = current_position_x + velocity_x + acceleration_x * sim_params.timestep * sim_params.timestep;

    // Y AXIS: Coulomb drag
    // TBD

    let next_position_y = current_position_y + velocity_y + sim_params.acceleration_y * sim_params.timestep * sim_params.timestep;


    // UPDATING OBJECT POSITION

    // Previous position is forgotten,
    
    // current position becomes previous position,
    verlet_object.previous_x = current_position_x;
    verlet_object.previous_y = current_position_y;

    // and next position becomes current position.
    verlet_object.current_x = next_position_x;
    verlet_object.current_y = next_position_y;
}

/// Simulation proper
fn verlet_simulation(
    time: Res<Time>,
    craft_params: Res<resources::SpacecraftParameters>,
    mut sim_params: ResMut<resources::SimulationParameters>,
    mut sail_query: Query<(&mut components::VerletObject, &components::Mass), With<components::SailElement>>,
    ) {

    // CALCULATION OF TIMESTEPS FOR THE CURRENT FRAME

    let timesteps = timestep_calculation(&time, &mut sim_params);

    if sim_params.debug { println!("New frame ------------------"); }

    for _ in 0..timesteps { 

        if sim_params.debug { println!("New timestep ---------------"); }

        // SIMULATION LOOP

        // Iterating over esail elements, in order.
        for element in craft_params.elements.iter() {

            // Getting information about the current sail element
            let (mut verlet_object, _) = sail_query.get_mut(*element).expect("No sail element found");

            if verlet_object.is_deployed {
                verlet_integration(&mut verlet_object, &mut sim_params, &craft_params);
            }
        }

        // CONSTRAINT LOOP

        for _ in 0..sim_params.iterations {

            if sim_params.debug { println!("New constraint iteration ---"); }

            for (index, sail_element) in craft_params.elements.iter().enumerate().skip(1) {

                // Information from current element
                //let current_verlet_object = sail_query.get(*sail_element).expect("No previous sail element found");
                let (current_verlet_object, _) = sail_query.get(*sail_element).expect("No previous sail element found");

                let current_element_x = current_verlet_object.current_x;
                let current_element_y = current_verlet_object.current_y;

                // Information from previous element
                let prev_sail_element = craft_params.elements[index - 1];
                //let prev_verlet_object = sail_query.get(prev_sail_element).expect("No previous sail element found");
                let (prev_verlet_object, _) = sail_query.get(prev_sail_element).expect("No previous sail element found");

                let prev_element_x = prev_verlet_object.current_x;
                let prev_element_y = prev_verlet_object.current_y;

                // Calculating distance between current sail element and previous element in the line
                let diff_x = current_element_x - prev_element_x;
                let diff_y = current_element_y - prev_element_y;
                let distance_between_elements = (diff_x * diff_x + diff_y * diff_y).sqrt();

                if sim_params.debug {
                    println!("Index: {} | Distance between elements: {}", index, distance_between_elements);
                }

                let mut difference = 0.0;

                if distance_between_elements > 0.0 {
                    // Don't get this formula really. Is it correct?
                    difference = (craft_params.resting_distance - distance_between_elements) / distance_between_elements;
                }

                // This shouldn't be .5 if one object is not deployed, although I believe it tends to the correct spot anyways.
                let correction_x = diff_x * 0.5 * difference;
                let correction_y = diff_y * 0.5 * difference;

                // UPDATING POSITIONS
                // Here's where, I think, I'll have to get the queries again.
                
                //let mut current_verlet_object = sail_query.get_mut(*sail_element).expect("No previous sail element found");
                let (mut current_verlet_object, _) = sail_query.get_mut(*sail_element).expect("No previous sail element found");
                
                if current_verlet_object.is_deployed {
                    current_verlet_object.current_x += correction_x;
                    current_verlet_object.current_y += correction_y;
                }

                let (mut prev_verlet_object, _) = sail_query.get_mut(prev_sail_element).expect("No previous sail element found");
                
                if prev_verlet_object.is_deployed {
                    prev_verlet_object.current_x -= correction_x;
                    prev_verlet_object.current_y -= correction_y;
                }
            }
        }
    }
}

fn update_transform_verlets(
    mut sail_query: Query<(&components::VerletObject, &mut Transform)>,
    ){
    
    for (verlet_object, mut transform) in sail_query.iter_mut() {
        transform.translation.x = verlet_object.current_x;
        transform.translation.y = verlet_object.current_y;
    }
} 

/// Updates position and visibility of the center of mass
fn update_center_of_mass(
    sim_params:     Res<resources::SimulationParameters>,
    mass_query:     Query<(&Transform, &components::Mass), Without<components::CenterOfMass>>,
    mut com_query:  Query<(&mut Transform, &mut Visibility), With<components::CenterOfMass>>, 
    ){

    let mut total_mass:     f32 = 0.0;
    let mut center_mass_x:  f32 = 0.0;
    let mut center_mass_y:  f32 = 0.0;

    for (transform, object_mass) in mass_query.iter() {
        total_mass += object_mass.0;
        center_mass_x += transform.translation.x * object_mass.0;
        center_mass_y += transform.translation.y * object_mass.0;
    }

    if sim_params.debug {
        println!("Total mass: {} | Center of mass: ({},{})", total_mass, center_mass_x, center_mass_y);
    }

    //let mut com_transform = com_query.single_mut();
    let (mut com_transform, mut com_visibility) = com_query.single_mut();

    com_transform.translation.x = center_mass_x;
    com_transform.translation.y = center_mass_y;

    com_visibility.is_visible = sim_params.com_visibility;
}
