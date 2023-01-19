use bevy::prelude::*;
use uom::si::f32 as quantities;  // Should I use f64?

#[derive(Component)]
pub struct CenterOfMass;

#[derive(Component, Debug)]
pub struct SailElement;

#[derive(Component, Debug)]
pub struct ESail {
    pub elements:           Vec<Entity>,
    pub resting_distance:   f32,    // meters
    //pub resting_distance:   quantities::Length,
}

#[derive(Component, Debug)]
pub struct Mass (
    pub quantities::Mass,
);

#[derive(Component)]
pub struct ElectricallyCharged {
    pub potential:  quantities::ElectricPotential,
}

// I could call this SailElement and make everything simpler
#[derive(Component, Debug)]
pub struct VerletObject {
    pub previous_x: f32,    // Wondering if this should stay in pixels or become physical meters
    pub previous_y: f32,
    pub current_x:  f32,
    pub current_y:  f32,
    pub is_deployed: bool,  // This would be better in another component, SailElement maybe
}
