use bevy::prelude::*;

use crate::{ components, resources };

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::gizmo_visibility)
            .add_system(Self::update_transform_verlets)     // Updates the position of the graphics
            ;
    }
}

impl GraphicsPlugin {

    //fn rotate_sat(){}

    //Update transform here

    fn gizmo_visibility (
        mut com_query:          Query<&mut Visibility, (With<components::CenterOfMass>, Without<components::Axes>)>, 
        mut axes_query:         Query<&mut Visibility, (With<components::Axes>, Without<components::CenterOfMass>)>,   
        simulation_parameters:  Res<resources::SimulationParameters>,
        ) {

        let mut com_entity  = com_query.single_mut();
        let mut axes_entity = axes_query.single_mut();

        com_entity.is_visible = simulation_parameters.com_visibility;
        axes_entity.is_visible = simulation_parameters.axes_visibility;
    }

    /// Updates the transform of the verlet objects after the simulation, so that the graphics get updated.
    /// Maybe it will need to update graphics of other things as well.
    fn update_transform_verlets(
        mut verlet_query: Query<(&components::VerletObject, &mut Transform)>,
        ){
        
        for (verlet_object, mut transform) in verlet_query.iter_mut() {
            transform.translation.x = verlet_object.current_x as f32;
            transform.translation.y = verlet_object.current_y as f32;
            transform.translation.z = verlet_object.current_z as f32;
        }

        // Should this update the rotation of the segments too?
    } 
}
