//https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_egui::EguiPlugin;
//use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
mod parameters;
mod elements;
use elements::ElementsPlugin;
mod physics;
use physics::PhysicsPlugin;
mod gui;
use gui::GUIPlugin;

extern crate uom;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })   // "Multi-Sample Anti-Aliasing"
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(ElementsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(EguiPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GUIPlugin)
        .add_startup_system(camera_system)
        .insert_resource(parameters::SpacecraftParameters{..Default::default()})
        .insert_resource(parameters::SolarWindParameters{..Default::default()})
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
