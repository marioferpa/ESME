use bevy::prelude::*;
use bevy::gltf::Gltf;

use uom::si::length::meter;

use crate::{ resources, graphics };

#[derive(Component)]
pub struct SatelliteBody;

pub fn spawn_cubesat(
    mut commands:           Commands,
    mut meshes:             ResMut<Assets<Mesh>>,
    mut materials:          ResMut<Assets<StandardMaterial>>,
    //imported_meshes:        Res<graphics::MeshHandles>,
    //assets_gltf: Res<Assets<Gltf>>, // Don't know what this is for
    spacecraft_parameters:  Res<super::SpacecraftParameters>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    ass: Res<AssetServer>,
    ) {

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

    let my_gltf = ass.load("cubesat.glb#Scene0");

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
