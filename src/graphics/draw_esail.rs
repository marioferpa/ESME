use bevy::prelude::*;

use uom::si::length::meter;

use crate::{ resources, spacecraft };

pub (super) fn update_esail_graphics (
    esail_query:            Query<&spacecraft::new_esail::NewESail>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    mut transform_query:    Query<&mut Transform>,
    balls_resource:         Res<super::Balls>,   // Need mut?
) {


    let esail = esail_query.single();

    for (index, verlet) in esail.elements.iter().enumerate() {


        let mut ball_transform =
            transform_query.get_mut(balls_resource.0[index]).unwrap();

        ball_transform.translation.x = 
            verlet.current_coordinates
                  .0[0]
                  .get::<meter>() as f32 * 
            simulation_parameters.pixels_per_meter as f32;

        ball_transform.translation.y = 
            verlet.current_coordinates
                  .0[1]
                  .get::<meter>() as f32 * 
            simulation_parameters.pixels_per_meter as f32;

        ball_transform.translation.z = 
            verlet.current_coordinates
                  .0[2]
                  .get::<meter>() as f32 * 
            simulation_parameters.pixels_per_meter as f32;
    }

}

pub (super) fn draw_new_esail (
    mut commands:           Commands,
    mut meshes:             ResMut<Assets<Mesh>>,
    mut materials:          ResMut<Assets<StandardMaterial>>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    mut balls_resource:     ResMut<super::Balls>,
    new_esail_query:        Query<&spacecraft::new_esail::NewESail>,
) {

    let new_esail = new_esail_query.get_single().unwrap();
    
    let sphere_radius = 2.5;   // 2.5 what? Apples?


    let mut sphere_storage: Vec<Entity> = Vec::new();
    
    for verlet_object in new_esail.elements.iter() {

        println!(
            "Verlet's x: {:?}", 
            verlet_object.current_coordinates.x().get::<meter>() as f32, 
        );

        let sphere =
            commands.spawn ( 

                PbrBundle {

                    mesh: meshes.add(
                        Mesh::from(
                            shape::UVSphere { 
                                radius: sphere_radius, 
                                ..default() 
                            }
                        )
                    ),

                    material: materials.add(
                        StandardMaterial {
                            base_color: Color::rgb(1.0, 0.0, 0.0),
                            ..Default::default()
                        }
                        .into(),
                    ),

                    transform: Transform::from_xyz(
                        verlet_object.current_coordinates.x().get::<meter>() as f32 * 
                            simulation_parameters.pixels_per_meter as f32, 
                        0.0,    // TODO
                        0.0     // TODO
                    ),
                    ..default()
                }
            ).id();

        sphere_storage.push(sphere);
    }

    // And now where can I store the sphere_storage?
    balls_resource.0 = sphere_storage;
    println!("Balls resource: {:?}", balls_resource);
}
