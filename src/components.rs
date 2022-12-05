use bevy::prelude::*;

#[derive(Component, Debug)]
// A 'tuple struct', I didn't know these existed.
pub struct Mass (
    pub f32
);

#[derive(Component)]
// Should this include the link with the previous item?
// Or what if every sailelement has an index, an integer, and it links to the previous one?
pub struct SailElement {
    //pub linked_to: Entity,
    pub index: i32,
    // resting_distance
}

// Needs a better name. VerletObject?
#[derive(Component)]
pub struct CanMove {
    pub previous_x: f32,
    pub previous_y: f32,
}