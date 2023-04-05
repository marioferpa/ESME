use bevy::prelude::*;

const ARROW_LENGTH:     f32 = 100.0;  // pixels?    Move to f64 for consistency?

#[derive(Component)]
pub struct Axes;

pub fn spawn_axes (
    mut commands:   Commands,
    mut meshes:     ResMut<Assets<Mesh>>,
    mut materials:  ResMut<Assets<StandardMaterial>>,
    ) {

    // Maybe I could make the axes an invisible entity, and the arrows children entities of that
    // one. Then I could rotate the parent and the children would follow.

    let axes_entity = commands.spawn(
        SpatialBundle { visibility: Visibility { is_visible: true }, ..Default::default() })
        .insert(Name::new("Axes"))
        .insert(Axes)
        .id(); 

    // X (red)
    let red         = Color::rgb(1.0, 0.0, 0.0);
    let x_direction = Vec3::new(1.0, 0.0, 0.0); 
    let x_rotation  = Quat::from_rotation_z(std::f32::consts::PI / 2.0);
    let x_arrow     = spawn_arrow(&mut commands, &mut meshes, &mut materials, red, ARROW_LENGTH, x_direction, x_rotation);
    commands.entity(axes_entity).push_children(&[x_arrow]);

    // Y (green)
    let green       = Color::rgb(0.0, 1.0, 0.0);
    let y_direction = Vec3::new(0.0, 1.0, 0.0); 
    let y_rotation  = Quat::from_rotation_y(0.0);
    let y_arrow     = spawn_arrow(&mut commands, &mut meshes, &mut materials, green, ARROW_LENGTH, y_direction, y_rotation);
    commands.entity(axes_entity).push_children(&[y_arrow]);

    // Z (blue)
    let blue        = Color::rgb(0.0, 0.0, 1.0);
    let z_direction = Vec3::new(0.0, 0.0, 1.0); 
    let z_rotation  = Quat::from_rotation_x(std::f32::consts::PI / 2.0);
    let z_arrow     = spawn_arrow(&mut commands, &mut meshes, &mut materials, blue, ARROW_LENGTH, z_direction, z_rotation);
    commands.entity(axes_entity).push_children(&[z_arrow]);
}

fn spawn_arrow (
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    color: Color, length: f32, direction: Vec3, rotation: Quat,
    ) -> Entity {

    let origin = direction * length / 2.0;

    let material = StandardMaterial { base_color: color, emissive: color, perceptual_roughness: 1.0, ..default() };

    return commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { 
            radius: 1.0, 
            depth:  length,
            ..default() })),
        material: materials.add(material),
        transform: Transform {
            translation: origin,
            rotation: rotation,
            ..default()
        },
        ..default()
    }).id();

}
