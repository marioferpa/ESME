//https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod components;
mod graphics;
use graphics::GraphicsPlugin;
mod physics;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })   // "Multi-Sample Anti-Aliasing"
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(camera_system)
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
