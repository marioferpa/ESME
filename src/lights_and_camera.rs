use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{ components };


// -------------------------- Camera plugin ---------------------------------------

pub struct LightsAndCameraPlugin;

impl Plugin for LightsAndCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup, (
                    Self::spawn_camera,
                    Self::spawn_light,
                )
            );
    }
}

//pub fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
pub fn get_primary_window_size (
    window_query: &Query<&Window, With<PrimaryWindow>>
) -> Vec2 {

    let window = window_query.get_single().unwrap();

    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    return window_size;
}

// TODO Take these out of the impl block
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

