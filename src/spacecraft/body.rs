use bevy::prelude::*;

use uom::si::length::meter;

use crate::{ resources };

#[derive(Component)]
pub struct SatelliteBody;

pub fn spawn_cubesat(
    mut commands:           Commands,
    spacecraft_parameters:  Res<super::SpacecraftParameters>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    assets:                 Res<AssetServer>,
    ) {

    println!("Cero"); // Not running

    let cubesat_size = spacecraft_parameters.body_size.get::<meter>() * simulation_parameters.pixels_per_meter as f64;

    //let cubesat_entity = commands.spawn(PbrBundle {
    //        mesh: meshes.add(Mesh::from(shape::Cube { size: cubesat_size as f32})),
    //        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //        transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //        ..default()
    //    }).id();

    //if let Some(gltf) = assets_gltf.get(&imported_meshes.0) {
    //    // spawn the first scene in the file
    //    commands.spawn(SceneBundle {
    //        scene: gltf.scenes[0].clone(),
    //        visibility: Visibility { is_visible: true },
    //        ..Default::default()
    //    });
    //}

    println!("Uno");

    let my_gltf = assets.load("cubesat.glb#Scene0");

    let cubesat_entity = commands.spawn(SceneBundle {
        scene: my_gltf,
        //transform: Transform::from_xyz(2.0, 0.0, -5.0),
        transform: Transform::from_scale(Vec3::new(cubesat_size as f32, cubesat_size as f32, cubesat_size as f32)),
        ..Default::default()
    }).id();


    commands.entity(cubesat_entity)
        .insert(Name::new("Satellite body"))
        .insert(SatelliteBody)
        ;

    println!("Cubesat spawned");
}
