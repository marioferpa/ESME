use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            ;
    }
}

fn spawn_camera(mut commands: Commands) {

    // 2D camera
    commands.spawn(Camera2dBundle::default());

    // 3D camera (test)
    //commands.spawn(Camera3dBundle { transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y), ..default() });
}
