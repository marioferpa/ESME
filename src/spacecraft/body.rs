use bevy::prelude::*;
use uom::si::length::meter;

use crate::{ resources };

#[derive(Component)]
pub struct SatelliteBody;

pub fn spawn_cubesat(
    mut commands:           Commands,
    mut meshes:             ResMut<Assets<Mesh>>,
    mut materials:          ResMut<Assets<StandardMaterial>>,
    spacecraft_parameters:  Res<super::SpacecraftParameters>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    ) {

    let cubesat_size = spacecraft_parameters.body_size.get::<meter>() * simulation_parameters.pixels_per_meter as f64;

    let cubesat_entity = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: cubesat_size as f32})),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }).id();

    commands.entity(cubesat_entity)
        .insert(Name::new("Satellite body"))
        .insert(SatelliteBody)
        //.insert(components::Mass(BODY_MASS))
        ;

    println!("Cubesat spawned");
}
