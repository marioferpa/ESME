use bevy::prelude::*;

#[derive(Component, Debug)]
// A 'tuple struct', I didn't know these existed.
pub struct Mass (
    pub f32
);

#[derive(Component)]
pub struct SailElement;
