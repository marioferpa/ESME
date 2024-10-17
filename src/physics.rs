// With a lot of help from https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519
// And https://toqoz.fyi/game-rope.html

// TODO Is this still happening?
// Problem, maybe: The simulation seems to be idle for the two first frames

use bevy::prelude::*;
use crate::{ components, resources, spacecraft };

use uom::si::*;

pub mod position_vector;
pub mod force_vector;
pub mod acceleration_vector;
pub mod verlet_object;

// All operations in this plugin should be done in physical units. Get rid of
// pixels in verlets. Graphics.rs should then translate distances to pixels when
// needed.

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                update_center_of_mass
            )
        ;
    }
}


fn update_center_of_mass (
    simulation_parameters:     Res<resources::SimulationParameters>,
    mass_query: Query<
        (&Transform, &components::Mass), 
        Without<spacecraft::center_mass::CenterOfMass>
    >,
    mut com_query: Query<
        &mut Transform, 
        With<spacecraft::center_mass::CenterOfMass>
    >, 
) {

    let mut total_mass:     f32 = 0.0;  // In this particular case I don't think I should use physical units.
                                        // Transform will be in pixels, and mass units are cancelled out.
    let mut center_mass_x:  f32 = 0.0;
    let mut center_mass_y:  f32 = 0.0;

    for (transform, object_mass) in mass_query.iter() {
        total_mass    += object_mass.0.value as f32; 
        center_mass_x += transform.translation.x * object_mass.0.value as f32;
        center_mass_y += transform.translation.y * object_mass.0.value as f32;
    }

    if simulation_parameters.debug {
        println!(
            "Total mass: {} | Center of mass: ({},{})", 
            total_mass, center_mass_x, center_mass_y
        );
    }

    let mut com_transform = com_query.single_mut();

    com_transform.translation.x = center_mass_x;
    com_transform.translation.y = center_mass_y;
}
