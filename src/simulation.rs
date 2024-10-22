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
                    verlet_simulation::verlet_simulation,
                    //voltage::update_esail_voltage // Not used anymore? TODO
                    rotate_body.before(verlet_simulation::verlet_simulation),   
                )
            )
        ;
    }
}


// TEST
// If this is going to move the esail's first element, it should run before the
// verlet simulation, right? TODO
fn rotate_body (
    mut body_query: Query<
        &mut Transform, 
        With<spacecraft::body::SatelliteBody>
    >,
    mut esail_query:    Query<&mut spacecraft::esail::ESail>,
    time:               Res<Time>, 
) {

    println!("");
    println!("First: rotate body and sail");

    let mut body_transform = body_query.single_mut();

    let mut esail = esail_query.single_mut();

    body_transform.rotate_z(time.delta_seconds() * 0.1);

    let mut first_verlet = &mut esail.elements[0];

    // Why's the value stuck? I would expect it to grow... It's like it is
    // overwritten or something.

    let rotated_coordinates = 
        first_verlet.current_coordinates.rotate_z(time.delta_seconds() * 0.1);

    first_verlet.update_coordinates(rotated_coordinates);

    // This works only if verlet_simulation is off, meaning that that overwrites
    // the position or something.
}
