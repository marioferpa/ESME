//https://gamedevelopment.tutsplus.com/tutorials/simulate-tearable-cloth-and-ragdolls-with-simple-verlet-integration--gamedev-520

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod components;
mod graphics;
mod gui;
mod physics;
mod resources;
mod settings;
mod simulation;
mod solar_wind;
mod spacecraft;
mod time;
mod user_input;

extern crate uom;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Msaa::Sample4)
        .insert_resource(solar_wind::SolarWind{..Default::default()})
        .insert_resource(resources::SimulationParameters{..Default::default()})

        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(graphics::GraphicsPlugin)
        .add_plugins(gui::GUIPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(simulation::SimulationPlugin)
        .add_plugins(spacecraft::SpacecraftPlugin)
        .add_plugins(user_input::UserInputPlugin)
        //.add_plugins(WorldInspectorPlugin::new())

        .add_plugins(FrameTimeDiagnosticsPlugin::default())

        .run();
}

