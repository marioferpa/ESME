use bevy::prelude::*;
use crate::{ components, resources, elements };

/// Updates the potential of every conductor to whatever the gui is showing
pub fn update_esail_voltage(
    spacecraft_parameters:  Res<elements::SpacecraftParameters>,
    mut electrical_query:   Query<&mut components::ElectricallyCharged>,
    ) {

    for mut electrical_element in electrical_query.iter_mut() {
        electrical_element.potential = spacecraft_parameters.wire_potential;
    }
}
