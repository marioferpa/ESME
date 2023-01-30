use bevy::prelude::*;

use crate::{ components, resources };

pub fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

// Idea for the coordinates: https://rustrepo.com/repo/urholaukkarinen-egui-gizmo

// -------------------------- Camera plugin ---------------------------------------

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_camera)
            ;
    }
}

impl CameraPlugin {

    fn spawn_camera(
        mut commands: Commands,
        simulation_parameters:  ResMut<resources::SimulationParameters>,
        ) {

        let translation = Vec3::new(-500.0, 100.0, 500.0);
        let radius = translation.length();

        if simulation_parameters.three_dimensions {
            // 3D camera (test)
            // I think quaternions can be used
            commands.spawn((
                Camera3dBundle { transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y), ..default() },
                components::PanOrbitCamera { radius, ..Default::default() },
                ));
        } else {
            // 2D camera
            commands.spawn(Camera2dBundle::default());
        }
    }
}

