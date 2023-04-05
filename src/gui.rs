use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{ resources, spacecraft };

use uom::si::*;

const MAX_VOLTAGE:  f64 = 30.0e3;   // Volts
const MAX_RPM:      f64 = 5.0;      // rpm

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
        mut sim_params:             ResMut<resources::SimulationParameters>,
        mut solar_wind:             ResMut<resources::SolarWindParameters>, 
        mut spacecraft_parameters:  ResMut<spacecraft::SpacecraftParameters>,
        mut esail_query:            Query<(&spacecraft::esail::ESail2, &mut Transform)>,
        ) {

        let (esail2, mut esail2_transform) = esail_query.single_mut();

        egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {

            ui.label("SPACECRAFT");

            ui.horizontal(|ui| { ui.label("Spacecraft rotation"); });
            ui.add(egui::Slider::new(&mut spacecraft_parameters.rpm.value, 0.0..=MAX_RPM).text("rpm"));

            ui.horizontal(|ui| { ui.label("Wire potential V_0"); });
            ui.add(egui::Slider::new(&mut spacecraft_parameters.wire_potential.value, 0.0..=MAX_VOLTAGE).text("V (want kV)"));

            ui.horizontal(|ui| {
                ui.label( format!("Deployed wire length: {} m", spacecraft_parameters.wire_length.get::<length::meter>()));
            });

            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut esail2_transform.translation.x, 0.0..=500.0).text("Deployed ESail2 (pixels?)"));
            });

            ui.separator();

            ui.label("SOLAR WIND");
            ui.horizontal(|ui| {
                ui.label("Electron temperature (eV)");
                // This works, shows it in the correct units, but why can't I mutate it now? 
                ui.add(egui::DragValue::new(&mut solar_wind.T_e.get::<energy::electronvolt>()));
            });

            ui.horizontal(|ui| {
                ui.label(format!("Solar wind velocity: {} km/s", solar_wind.velocity.get::<velocity::kilometer_per_second>()));
            });

            ui.separator();


            ui.label("SIMULATION");

            ui.horizontal(|ui| { ui.label("Constraint iterations per timestep"); });
            ui.add(egui::Slider::new(&mut sim_params.iterations, 1..=100).text("Iterations"));


            ui.horizontal(|ui| { 
                ui.checkbox(&mut sim_params.debug, "Debug mode");
            });

            ui.horizontal(|ui| { 
                ui.checkbox(&mut sim_params.com_visibility, "Show center of mass");
            });

            ui.horizontal(|ui| { 
                ui.checkbox(&mut sim_params.axes_visibility, "Show axes");
            });

            ui.separator();

            ui.label("RESULTS");
            ui.horizontal(|ui| { 
                ui.label( format!("Total coulomb force: "));
            });

            //if ui.add(egui::Button::new("Reset")).clicked() {
            //   println!("Hey");
            //}
        });
    }
}
