//https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_egui::EguiPlugin;

mod components;
mod resources;
mod graphics;
use graphics::GraphicsPlugin;
mod physics;
use physics::PhysicsPlugin;
mod gui;
use gui::GUIPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })   // "Multi-Sample Anti-Aliasing"
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(GUIPlugin)
        .add_startup_system(camera_system)
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
