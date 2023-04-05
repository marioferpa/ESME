use bevy::prelude::*;

#[derive(Component)]
pub struct CenterOfMass;

pub fn spawn_center_mass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ){

    let com_entity = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 10.0, ..default() })),
            material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }).id();

    commands.entity(com_entity).insert(CenterOfMass);
}
