//https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-519

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .insert_resource(Msaa::Sample4)   // "Multi-Sample Anti-Aliasing"
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(graphics::GraphicsPlugin)
        .add_plugins(gui::GUIPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(lights_and_camera::LightsAndCameraPlugin)
        .add_plugins(simulation::SimulationPlugin)
        .add_plugins(spacecraft::SpacecraftPlugin)
        .add_plugins(user_input::UserInputPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(solar_wind::SolarWind{..Default::default()})
        .insert_resource(resources::SimulationParameters{..Default::default()})
        .run();
}
