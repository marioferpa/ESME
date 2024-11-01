use bevy::prelude::*;

use uom::si::length::meter;

use crate::{ resources, spacecraft };

pub (super) fn update_esail_graphics (
    esail_query:            Query<&spacecraft::esail::ESail>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    mut transform_query:    Query<&mut Transform>,
    mut balls_resource:         ResMut<super::Balls>,

    mut sail_event:         EventReader<spacecraft::esail::SailExtended>,

    mut commands:       Commands,
    mut meshes:         ResMut<Assets<Mesh>>,
    mut materials:      ResMut<Assets<StandardMaterial>>,
) {


    let esail = esail_query.single();

    for event in sail_event.read() { // Should be only one

        // Despawn old balls

        delete_balls(
            &mut commands,
            &mut balls_resource
        );

        // Don't add a new ball, redraw the whole thing instead
        //add_new_ball(
        //    &mut commands, 
        //    &mut balls_resource, 
        //    &mut meshes, 
        //    &mut materials
        //);

        draw_esail(
            &mut balls_resource,
            &mut commands,
            &esail,
            &mut meshes,
            &mut materials,
            &simulation_parameters
        );

        // Test, because I think that the change doesn't happen fast enough or
        // something and the next step can't find the entities it needs
        return
    }

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

// Test
//fn add_new_ball(
//    commands:           &mut Commands,
//    mut balls_resource: &mut ResMut<super::Balls>,
//    mut meshes:         &mut ResMut<Assets<Mesh>>,
//    mut materials:      &mut ResMut<Assets<StandardMaterial>>,
//) {
//
//    // Trying to insert in second position, as in extend_esail()
//
//    if balls_resource.0.len() > 1 {
//        if let Some(second_ball) = balls_resource.0.get(1).cloned() {
//            balls_resource.0.insert(1, second_ball);
//        }
//    }
//}

fn delete_balls (
    mut commands:           &mut Commands,
    mut balls_resource:     &mut ResMut<super::Balls>,
) {

    for ball_entity in &balls_resource.0 {

        commands.entity(*ball_entity).despawn();
    }
}

fn draw_esail (
    mut balls_resource:     &mut ResMut<super::Balls>,
    mut commands:           &mut Commands,
    esail:                  &spacecraft::esail::ESail,
    mut meshes:             &mut ResMut<Assets<Mesh>>,
    mut materials:          &mut ResMut<Assets<StandardMaterial>>,
    simulation_parameters:  &Res<resources::SimulationParameters>,
) {

    let sphere_radius = 2.5;   // 2.5 what? Apples?

    let mut sphere_storage: Vec<Entity> = Vec::new();
    
    for verlet_object in esail.elements.iter() {

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
                        verlet_object.current_coordinates.y().get::<meter>() as f32 * 
                            simulation_parameters.pixels_per_meter as f32, 
                        verlet_object.current_coordinates.z().get::<meter>() as f32 * 
                            simulation_parameters.pixels_per_meter as f32, 
                    ),
                    ..default()
                }
            ).id();

        sphere_storage.push(sphere);
    }

    balls_resource.0 = sphere_storage;
    println!("Balls resource: {:?}", balls_resource);
}

// FIXME This only runs suring Startup, that's why the new balls are invisible

pub (super) fn first_esail_draw (
    mut commands:           Commands,
    mut meshes:             ResMut<Assets<Mesh>>,
    mut materials:          ResMut<Assets<StandardMaterial>>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    mut balls_resource:     ResMut<super::Balls>,
    esail_query:            Query<&spacecraft::esail::ESail>,
) {

    let esail = esail_query.get_single().unwrap();

    draw_esail(
        &mut balls_resource,
        &mut commands,
        &esail,
        &mut meshes,
        &mut materials,
        &simulation_parameters
    );
}
