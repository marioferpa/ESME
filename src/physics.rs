use bevy::prelude::*;
use crate::{ components };

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_positions); // Test function
    }
}

impl PhysicsPlugin {

    fn update_positions(
        //mut commands: Commands,
        mut sail_query: Query<(&components::SailElement, &mut Transform)>
        ) {

        let translation = Vec3::new(0.0, 0.1, 0.0);

        for (_element, mut transform) in sail_query.iter_mut() {
            transform.translation += translation;
        }
    }
}
