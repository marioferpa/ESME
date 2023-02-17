use bevy::prelude::*;

use crate::{ components, resources };

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::gizmo_visibility)
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
}
