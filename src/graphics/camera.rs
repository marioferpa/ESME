use bevy::prelude::*;

#[derive(Component)]
pub struct Camera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

pub (super) fn spawn_camera(
    mut commands: Commands,
) {

    let translation = Vec3::new(-500.0, 100.0, 500.0);
    let radius = translation.length();

    // 3D camera 
    commands.spawn((
        Camera3dBundle { transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y), ..default() },
        Camera { radius, ..Default::default() },
        ));
    
}
