use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use uom::si::length::meter;

use crate::{ physics, spacecraft, resources };

mod axes;
pub mod camera;
mod lights;
mod load_models;
mod draw_esail;

// Test: storing the esail "balls" in a resource
#[derive(Debug, Resource)]
struct Balls (Vec<Entity>); 

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Balls(Vec::new()))
            .add_systems(
                PreStartup, 
                load_models::load_models
            )
            .add_systems(
                Startup, (
                    axes::spawn_axes,
                    camera::spawn_camera,
                    lights::spawn_light,
                    draw_esail::draw_new_esail,
                )
            )
            .add_systems(
                Update, (
                    gizmo_visibility,
                    //update_transform_verlets,   //Replaced by update_new_esail
                    update_rotation_axes
                )
            )
            .add_systems(
                PostUpdate,
                draw_esail::update_esail_graphics
            )
        ;
    }
}


// At some point I think that physics.rs will update a component containing the rotation, and
// this will adapt the Transform to the value of that component.

fn gizmo_visibility (
    mut com_query:          Query<&mut Visibility, (With<spacecraft::center_mass::CenterOfMass>, Without<axes::Axes>)>, 
    mut axes_query:         Query<&mut Visibility, (With<axes::Axes>, Without<spacecraft::center_mass::CenterOfMass>)>,   
    simulation_parameters:  Res<resources::SimulationParameters>,
    ) {

    let mut com_visibility  = com_query.single_mut();
    let mut axes_visibility = axes_query.single_mut();

    if simulation_parameters.com_visibility {
        *com_visibility = Visibility::Visible
    } else {
        *com_visibility = Visibility::Hidden
    }

    //axes_entity.is_visible = simulation_parameters.axes_visibility;
    if simulation_parameters.axes_visibility {
        *axes_visibility = Visibility::Visible
    } else {
        *axes_visibility = Visibility::Hidden
    }
}



//fn update_transform_verlets (
//    mut verlet_query: Query<(&physics::verlet_object::VerletObject, &mut Transform)>,
//    simulation_parameters:  Res<resources::SimulationParameters>,
//){
//    
//    for (verlet_object, mut transform) in verlet_query.iter_mut() {
//        // Should I use get<meter> in these cases?
//        transform.translation.x = verlet_object.current_coordinates.0[0].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
//        transform.translation.y = verlet_object.current_coordinates.0[1].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
//        transform.translation.z = verlet_object.current_coordinates.0[2].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
//
//        //println!("Transform X: {}", transform.translation.x);
//    }
//} 



fn update_rotation_axes (
    mut axes_query:         Query<&mut Transform, (With<axes::Axes>, Without<spacecraft::body::SatelliteBody>)>,   
    satellite_query:    Query<&Transform, (With<spacecraft::body::SatelliteBody>, Without<axes::Axes>)>,
) {

    let mut axes_transform  = axes_query.single_mut();
    let satellite_transform = satellite_query.single();

    axes_transform.rotation = satellite_transform.rotation;
}

pub fn get_primary_window_size (
    window_query: &Query<&Window, With<PrimaryWindow>>
) -> Vec2 {

    let window = window_query.get_single().unwrap();

    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    return window_size;
}
