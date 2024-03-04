use bevy::prelude::*;

use uom::si::length::meter;

use crate::{ resources, spacecraft };

// I think I see the problem that past me was trying to solve. I have a vector
// of VerletObject, and a sphere needs to be spawned and moved for each Verlet.
// Problem is that if I just update this function and put a sphere where it
// belongs, I would be spawning new spheres constantly instead of moving them. I
// would of course prefer to move them, but then how do I track which sphere
// should go where?

// If I keep the spheres in a vector, as I was doing, and I know that this
// vector is as long as the deployed verlets, I can be sure that each index
// corresponds to the sphere of the same index. Does that work when reeling in
// and out though?
//
// It should be the same I guess. If I add an item to the start of the esail and
// do the same thing to the spheres then the index should keep working

// This is drawing once, spawning the spheres. I guess I am missing a function
// that updates it
pub (super) fn draw_new_esail (
    mut commands:           Commands,
    mut meshes:             ResMut<Assets<Mesh>>,
    mut materials:          ResMut<Assets<StandardMaterial>>,
    simulation_parameters:  Res<resources::SimulationParameters>,
    new_esail_query:        Query<&spacecraft::new_esail::NewESail>,
) {

    let new_esail = new_esail_query.get_single().unwrap();
    
    let sphere_radius = 2.5;   // 2.5 what? Apples?


    let mut sphere_storage: Vec<Entity> = Vec::new();
    
    for verlet_object in new_esail.deployed_elements.iter() {

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

        // TODO: What the hell is this?
        sphere_storage.push(sphere);

        // And now where can I store the sphere_storage?
    }
}