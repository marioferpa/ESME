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

    let cubesat_size = 
        spacecraft_parameters.body_size.get::<meter>() * 
        simulation_parameters.pixels_per_meter as f64;


    let my_gltf = assets.load("cubesat.glb#Scene0");


    let cubesat_entity = commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_scale(
            Vec3::new(
                cubesat_size as f32, 
                cubesat_size as f32, 
                cubesat_size as f32
            )
        ),
        ..Default::default()
    }).id();


    commands
        .entity(cubesat_entity)
        .insert(Name::new("Satellite body"))
        .insert(SatelliteBody)
        ;

    println!("Cubesat spawned");
}
