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
                    //voltage::update_esail_voltage
                    rotate_body,
                )
            )
        ;
    }
}


// TEST
fn rotate_body (
    mut body_query: Query<
        &mut Transform, 
        With<spacecraft::body::SatelliteBody>
    >,
    time:       Res<Time>, 
) {

    let mut body_transform = body_query.single_mut();

    body_transform.rotate_z(time.delta_seconds() * 0.5);
}
