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
        mut craft_params: ResMut<resources::SpacecraftParameters>,
        ) {

        egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {

            // Title
            ui.heading("Options");

            // Sliders
            ui.horizontal(|ui| { ui.label("Spacecraft rotation"); });
            ui.add(egui::Slider::new(&mut craft_params.rpm, 0..=100).text("rpm"));

            ui.horizontal(|ui| { ui.label("'Coulomb' acceleration (y)"); });
            ui.add(egui::Slider::new(&mut sim_params.acceleration_y, -100.0..=100.0).text("value"));

            ui.horizontal(|ui| { ui.label("Wire potential V_0"); });
            ui.add(egui::Slider::new(&mut craft_params.wire_potential, 0.0..=100.0).text("V"));

            ui.separator();

            ui.horizontal(|ui| { ui.label("Constraint iterations per timestep"); });
            ui.add(egui::Slider::new(&mut sim_params.iterations, 1..=100).text("Iterations"));

            ui.separator();

            ui.horizontal(|ui| { 
                ui.checkbox(&mut sim_params.debug, "Debug mode");
            });

            ui.horizontal(|ui| { 
                //ui.toggle_value(&mut sim_params.center_of_mass, "Show center of mass");
                ui.checkbox(&mut sim_params.com_visibility, "Show center of mass");
            });

            //if ui.add(egui::Button::new("Reset")).clicked() {
            //   println!("Hey");
            //}
        });
    }
}
