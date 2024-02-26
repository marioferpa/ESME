//https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
mod lights_and_camera;
mod graphics;
mod gui;
mod physics;
mod resources;
mod simulation;
mod solar_wind;
mod spacecraft;
mod user_input;

extern crate uom;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })   // "Multi-Sample Anti-Aliasing"
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(gui::GUIPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(lights_and_camera::LightsAndCameraPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(simulation::SimulationPlugin)
        .add_plugin(spacecraft::SpacecraftPlugin)
        .add_plugin(user_input::UserInputPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(solar_wind::SolarWind{..Default::default()})
        .insert_resource(resources::SimulationParameters{..Default::default()})
        .run();
}
