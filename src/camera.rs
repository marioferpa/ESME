use bevy::prelude::*;

use crate::{ resources };

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            ;
    }
}

fn spawn_camera(
    mut commands: Commands,
    simulation_parameters:  ResMut<resources::SimulationParameters>,
    ) {

    if simulation_parameters.three_dimensions {
        // 3D camera (test)
        // I think quaternions can be used
        commands.spawn(Camera3dBundle { transform: Transform::from_xyz(-500.0, 100.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y), ..default() });
    } else {
        // 2D camera
        commands.spawn(Camera2dBundle::default());
    }
}
