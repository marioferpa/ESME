// Move the simulation plugin and the resource. Leave physics.rs only with use
// position_vector, use etc
use bevy::prelude::*;
use crate::spacecraft;

mod verlet_simulation;
mod voltage;

pub struct SimulationPlugin;

// TODO Make solar wind affect the new sail. Next, try to introduce the spring
// recovery force 
// Maybe they have no voltage?

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, (
                    rotate_body
                        .before(verlet_simulation::verlet_simulation),   
                    verlet_simulation::verlet_simulation,
                    //voltage::update_esail_voltage // Not used anymore? TODO
                )
            )
        ;
    }
}


fn rotate_body (
    mut body_query: Query<
        &mut Transform, 
        With<spacecraft::body::SatelliteBody>
    >,
    mut esail_query:    Query<&mut spacecraft::esail::ESail>,
    time:               Res<Time>, 
) {

    let speed = 0.5;    // No units, just a multiplier

    let mut body_transform = body_query.single_mut();

    let mut esail = esail_query.single_mut();

    body_transform.rotate_z(time.delta_seconds() * speed);

    let mut first_verlet = &mut esail.elements[0];


    let rotated_coordinates = 
        first_verlet.current_coordinates.rotate_z(time.delta_seconds() * speed);

    first_verlet.update_coordinates(rotated_coordinates);
}
