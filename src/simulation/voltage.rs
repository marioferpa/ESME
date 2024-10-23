//use bevy::prelude::*;
//use crate::{ components, spacecraft };
//
//pub fn update_esail_voltage (
//    spacecraft_parameters:  Res<spacecraft::SpacecraftParameters>,
//    mut electrical_query:   Query<&mut components::ElectricallyCharged>,
//) {
//
//    for mut electrical_element in electrical_query.iter_mut() {
//
//        //println!("Updating charge of component {:?}", electrical_element);
//        electrical_element.potential = spacecraft_parameters.tether_potential;
//    }
//}
