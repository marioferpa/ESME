use bevy::prelude::*;

use crate::{ components };

pub fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

// -------------------------- Camera plugin ---------------------------------------

pub struct LightsAndCameraPlugin;

impl Plugin for LightsAndCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_camera)
            .add_startup_system(Self::spawn_light)
            ;
    }
}

impl LightsAndCameraPlugin {

    fn spawn_camera(
        mut commands: Commands,
        ) {

        let translation = Vec3::new(-500.0, 100.0, 500.0);
        let radius = translation.length();

        // 3D camera 
        commands.spawn((
            Camera3dBundle { transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y), ..default() },
            components::PanOrbitCamera { radius, ..Default::default() },
            ));
        
    }

    fn spawn_light( mut commands: Commands) {
        commands.spawn(DirectionalLightBundle {
            ..default()
        });
    }
}

