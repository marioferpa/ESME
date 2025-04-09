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

        delete_balls(
            &mut commands,
            &mut balls_resource
        );

        draw_esail(
            &mut balls_resource,
            &mut commands,
            &esail,
            &mut meshes,
            &mut materials,
            &simulation_parameters
        );

        return
    }

    //for (index, verlet) in esail.elements.iter().enumerate() {
    for (index, rk) in esail.rk_objects.iter().enumerate() {

        let mut ball_transform =
            transform_query.get_mut(balls_resource.0[index]).unwrap();

        ball_transform.translation.x = 
            //verlet.current_coordinates
            rk.position
                  .0[0]
                  .get::<meter>() as f32 * 
            simulation_parameters.pixels_per_meter as f32;

        ball_transform.translation.y = 
            //verlet.current_coordinates
            rk.position
                  .0[1]
                  .get::<meter>() as f32 * 
            simulation_parameters.pixels_per_meter as f32;

        ball_transform.translation.z = 
            //verlet.current_coordinates
            rk.position
                  .0[2]
                  .get::<meter>() as f32 * 
            simulation_parameters.pixels_per_meter as f32;
    }

}


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
    
    //for verlet_object in esail.elements.iter() {
    for rk_object in esail.rk_objects.iter() {

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
                            //base_color: Color::rgb(1.0, 0.0, 0.0),
                            base_color: Color::rgb(0.0, 1.0, 0.0),
                            ..Default::default()
                        }
                        .into(),
                    ),

                    transform: Transform::from_xyz(
                        //verlet_object.current_coordinates.x().get::<meter>() as f32 * 
                        rk_object.position.x().get::<meter>() as f32 * 
                            simulation_parameters.pixels_per_meter as f32, 
                        //verlet_object.current_coordinates.y().get::<meter>() as f32 * 
                        rk_object.position.y().get::<meter>() as f32 * 
                            simulation_parameters.pixels_per_meter as f32, 
                        //verlet_object.current_coordinates.z().get::<meter>() as f32 * 
                        rk_object.position.z().get::<meter>() as f32 * 
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
