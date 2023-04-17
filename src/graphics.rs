use bevy::prelude::*;
use bevy::gltf::Gltf;

use uom::si::length::meter;

use crate::{ physics, spacecraft, resources };

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, Self::load_models)
            .add_system(Self::gizmo_visibility)
            .add_system(Self::update_transform_verlets)     // Updates the position of the graphics
            .add_system(Self::update_rotation_axes)
            ;
    }
}

#[derive(Resource)]
pub struct MeshHandles(pub Handle<Gltf>);

impl GraphicsPlugin {

    fn load_models (
        mut commands: Commands,
        assets: Res<AssetServer>,
        ) {

        let cubesat_model = assets.load("cubesat.glb");
        commands.insert_resource(MeshHandles(cubesat_model));
        println!("Mesh loaded");
    }

    // At some point I think that physics.rs will update a component containing the rotation, and
    // this will adapt the Transform to the value of that component.

    fn gizmo_visibility (
        mut com_query:          Query<&mut Visibility, (With<spacecraft::center_mass::CenterOfMass>, Without<spacecraft::axes::Axes>)>, 
        mut axes_query:         Query<&mut Visibility, (With<spacecraft::axes::Axes>, Without<spacecraft::center_mass::CenterOfMass>)>,   
        simulation_parameters:  Res<resources::SimulationParameters>,
        ) {

        let mut com_entity  = com_query.single_mut();
        let mut axes_entity = axes_query.single_mut();

        com_entity.is_visible = simulation_parameters.com_visibility;
        axes_entity.is_visible = simulation_parameters.axes_visibility;
    }

    /// Updates the transform of the verlet objects after the simulation, so that the graphics get updated.
    fn update_transform_verlets (
        mut verlet_query: Query<(&physics::verlet_object::VerletObject, &mut Transform)>,
        simulation_parameters:  Res<resources::SimulationParameters>,
        ){
        
        for (verlet_object, mut transform) in verlet_query.iter_mut() {
            // Should I use get<meter> in these cases?
            transform.translation.x = verlet_object.current_coordinates.0[0].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
            transform.translation.y = verlet_object.current_coordinates.0[1].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
            transform.translation.z = verlet_object.current_coordinates.0[2].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;

            //println!("Transform X: {}", transform.translation.x);
        }
    } 

    fn update_rotation_axes (
        mut axes_query:         Query<&mut Transform, (With<spacecraft::axes::Axes>, Without<spacecraft::body::SatelliteBody>)>,   
        satellite_query:    Query<&Transform, (With<spacecraft::body::SatelliteBody>, Without<spacecraft::axes::Axes>)>,
        ) {

        let mut axes_transform  = axes_query.single_mut();
        let satellite_transform = satellite_query.single();

        axes_transform.rotation = satellite_transform.rotation;
    }
}
