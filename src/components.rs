use bevy::prelude::*;

#[derive(Component)]
pub struct CenterOfMass;

#[derive(Component, Debug)]
// A 'tuple struct', I didn't know these existed.
pub struct Mass (
    pub f32
);

#[derive(Component, Debug)]
// Should this include the link with the previous item?
// Or what if every sailelement has an index, an integer, and it links to the previous one?
pub struct SailElement {
    //pub index: i32,
    pub is_deployed: bool,
}

// I could call this SailElement and make everything simpler
#[derive(Component, Debug)]
pub struct VerletObject {
    pub previous_x: f32,
    pub previous_y: f32,
    pub current_x:  f32,
    pub current_y:  f32,
    pub is_deployed: bool,
}
