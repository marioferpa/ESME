use bevy::prelude::*;
use uom::si::f64 as quantities;  
use uom::si::electric_potential::volt;

#[derive(Component, Debug)]
pub struct Mass (
    pub quantities::Mass,
);

//TODO Check if this is actually in use
#[derive(Component, Debug)]
pub struct ElectricallyCharged {
    pub potential:  quantities::ElectricPotential,
}

impl Default for ElectricallyCharged {
    fn default() -> Self {
        ElectricallyCharged {
            potential: quantities::ElectricPotential::new::<volt>(0.0),
        }
    }
}
 
//#[derive(Component)]
//pub struct Position (
//    pub Vec<quantities::Length>,
//);
