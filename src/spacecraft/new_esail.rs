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
    pub origin:     PositionVector, 
    pub elements:   Vec<VerletObject>,  
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

    let mut elements: Vec<VerletObject> = Vec::new();

    let zero =  quantities::Length::new::<length::meter>(0.0);


    for number in 0.. number_of_elements {

        let x = spacecraft_parameters.esail_origin.x() + 
            spacecraft_parameters.segment_length() * number as f64;

        let is_deployed = if number == 0 {
            false 
        } else {
            true
        };


        let verlet = VerletObject {  
            previous_coordinates:   PositionVector::new(x, zero, zero),
            current_coordinates:    PositionVector::new(x, zero, zero),
            is_deployed,
            current_force:          ForceVector::empty(),
        };
        
        elements.push(verlet);
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
                elements: elements,
            }
        )
        // TODO This doesn't do anything
        //.insert(components::ElectricallyCharged{ ..Default::default() })
        ;

    println!("(New) E-sail spawned");
}
