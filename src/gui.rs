use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{ resources };

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::sidebar)
            ;
    }
}

impl GUIPlugin {

    fn sidebar(
        mut egui_ctx: ResMut<EguiContext>,
        mut sim_params: ResMut<resources::SimulationParameters>,
        ) {
        egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {

            // Title
            ui.heading("Options");

            // Sliders
            ui.horizontal(|ui| { ui.label("'Centrifugal' acceleration (x)"); });
            ui.add(egui::Slider::new(&mut sim_params.acceleration_x, 0.0..=10000.0).text("value"));

            ui.horizontal(|ui| { ui.label("'Coulomb' acceleration (y)"); });
            ui.add(egui::Slider::new(&mut sim_params.acceleration_y, -10000.0..=10000.0).text("value"));

            ui.separator();
        });
    }
}