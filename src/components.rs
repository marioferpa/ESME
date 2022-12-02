use bevy::prelude::*;

#[derive(Component, Debug)]
// A 'tuple struct', I didn't know these existed.
pub struct Mass (
    pub f32
);

#[derive(Component)]
pub struct SailElement;

// Needs a better name
#[derive(Component)]
pub struct CanMove {
    pub previous_x: f32,
    pub previous_y: f32,
}
