use bevy::prelude::*;
use bevy::gltf::Gltf;

#[derive(Resource)]
pub struct MeshHandles(pub Handle<Gltf>);

pub (super) fn load_models (
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let cubesat_model = assets.load("cubesat.glb");

    commands.insert_resource(MeshHandles(cubesat_model));

    println!("Cubesat mesh loaded");
}
