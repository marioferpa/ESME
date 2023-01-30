use bevy::prelude::*;
use uom::si::f64 as quantities;  // Should I use f64?

#[derive(Component)]
pub struct CenterOfMass;

#[derive(Component, Debug)]
//pub struct SailElement;
pub struct SailElement {
    pub is_deployed:    bool,   // Not used. Makes more sense than in VerletObject,
                                // but it's harder to access from the code.
}

#[derive(Component, Debug)]
pub struct ESail {
    pub elements: Vec<Entity>,
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
    pub previous_x:     f64,
    pub previous_y:     f64,
    pub current_x:      f64,
    pub current_y:      f64,
    pub is_deployed:    bool,  // This would be better in another component, SailElement maybe
}
