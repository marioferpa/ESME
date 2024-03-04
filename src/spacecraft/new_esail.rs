use bevy::prelude::*;

use uom::si::*;
use uom::si::f64 as quantities;  
use uom::si::length::meter;

use crate::{ components, physics, resources };

use physics::force_vector::ForceVector as ForceVector;
use physics::verlet_object::VerletObject as VerletObject;
use physics::position_vector::PositionVector as PositionVector;

#[derive(Component)]
pub struct NewESail {  
    pub origin:                 PositionVector, 
    //pub undeployed_elements:    Vec<VerletObject>,
    pub deployed_elements:      Vec<VerletObject>,
}



pub fn spawn_new_esail (
    mut commands:           Commands,
    mut meshes:             ResMut<Assets<Mesh>>,
    mut materials:          ResMut<Assets<StandardMaterial>>,
    spacecraft_parameters:  Res<super::SpacecraftParameters>,
) {

    let esail_entity = commands.spawn(
        SpatialBundle { 
            visibility: Visibility::Visible,
            ..Default::default() 
        })
        .insert(Name::new("New E-sail"))
        .id();

    //let number_of_elements = spacecraft_parameters.number_of_esail_elements();
    let number_of_elements = 10; 

    let mut deployed_elements: Vec<VerletObject> = Vec::new();

    let zero =  quantities::Length::new::<length::meter>(0.0);


    // TODO First element should be undeployed? Or do I have an anchor?

    for number in 0.. number_of_elements {

        let x = spacecraft_parameters.esail_origin.x() + 
            spacecraft_parameters.segment_length() * number as f64;

        let verlet = VerletObject {  
            previous_coordinates:   PositionVector::new(x, zero, zero),
            current_coordinates:    PositionVector::new(x, zero, zero),
            is_deployed:            true,
            current_force:          ForceVector::empty(),
        };
        
        deployed_elements.push(verlet);
    }

    //println!("New ESail: {:?}", deployed_elements);

    commands.entity(esail_entity)
        .insert(
            NewESail {
                origin: PositionVector::new(
                    spacecraft_parameters.esail_origin.x(),
                    zero,
                    zero
                ),
                deployed_elements:      deployed_elements,
            }
        )
        // TODO This was meant for individual verlets, maybe won't work here
        .insert(components::ElectricallyCharged{ ..Default::default() })
        ;

    println!("(New) E-sail spawned");
}






//pub fn update_new_esail_graphics (
//    mut new_esail_query:    Query<&mut NewESail>,
//) {
//
//    let new_esail = new_esail_query.single();
//
//    //for (verlet_object, mut transform) in verlet_query.iter_mut() {
//    //    // Should I use get<meter> in these cases?
//    //    transform.translation.x = verlet_object.current_coordinates.0[0].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
//    //    transform.translation.y = verlet_object.current_coordinates.0[1].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
//    //    transform.translation.z = verlet_object.current_coordinates.0[2].get::<meter>() as f32 * simulation_parameters.pixels_per_meter as f32;
//
//    //    //println!("Transform X: {}", transform.translation.x);
//    //}
//
//    // So previously every object on the sail had verlet coordinates and a transform, and the
//    // transform was updated every frame to that of the verlet object. Now I can't do that.
//    // Now I have to go over all the verlets, and all the spheres (I need to store them somewhere
//    // too, ugh), and match their positions in pairs.
//
//    for verlet_object in &new_esail.deployed_elements {
//        //println!("Current coordinates: {:?}", verlet_object.current_coordinates);
//    } 
//}
//
//
//
