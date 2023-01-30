//https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_egui::EguiPlugin;
//use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
mod resources;
mod graphics;
use graphics::GraphicsPlugin;
mod physics;
use physics::PhysicsPlugin;
mod gui;
use gui::GUIPlugin;
mod camera;
use camera::CameraPlugin;

extern crate uom;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })   // "Multi-Sample Anti-Aliasing"
        //.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(EguiPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GUIPlugin)
        .add_plugin(CameraPlugin)
        .insert_resource(resources::SpacecraftParameters{..Default::default()})
        .insert_resource(resources::SolarWindParameters{..Default::default()})
        .insert_resource(resources::SimulationParameters{..Default::default()})
        .run();
}
