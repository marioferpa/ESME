use bevy::prelude::*;
use uom::si::f64 as quantities;  // Should I use f64?

#[derive(Component)]
pub struct CenterOfMass;

#[derive(Component)]
pub struct Axes;

#[derive(Component, Debug)]
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

/// Tags an entity as capable of panning and orbiting. Taken from Bevy cheatbook
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}
