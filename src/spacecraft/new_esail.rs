use bevy::prelude::*;

use uom::si::*;
use uom::si::f64 as quantities;

use crate::{ physics };
use physics::force_vector::ForceVector as ForceVector;
use physics::verlet_object::VerletObject as VerletObject;
use physics::position_vector::PositionVector as PositionVector;

#[derive(Component)]
pub struct NewESail {  
    pub origin:                 PositionVector, 
    pub undeployed_elements:    Vec<VerletObject>,
    pub deployed_elements:      Vec<VerletObject>,
}

pub fn spawn_new_esail(
    mut commands:   Commands,
    mut meshes:     ResMut<Assets<Mesh>>,
    mut materials:  ResMut<Assets<StandardMaterial>>,
    spacecraft_parameters: Res<super::SpacecraftParameters>,
) {

    let esail_entity = commands.spawn(
        SpatialBundle { 
            visibility: Visibility {
                is_visible: true 
            }, 
            ..Default::default() 
        })
        .insert(Name::new("New E-sail"))
        .id();

    //let number_of_elements = spacecraft_parameters.number_of_esail_elements();
    let number_of_elements = 3;

    let mut deployed_elements:    Vec<VerletObject> = Vec::new();

    let x    =  spacecraft_parameters.esail_origin.x();
    let zero =  quantities::Length::new::<length::meter>(0.0);

    for number in 0.. number_of_elements - 1 {

        let verlet = VerletObject {  
            previous_coordinates:   PositionVector::new(x, zero, zero),
            current_coordinates:    PositionVector::new(x, zero, zero),
            is_deployed:            false,
            current_force:          ForceVector::empty(),
        };
        
        deployed_elements.push(verlet);
    }

    println!("New ESail: {:?}", deployed_elements);

    commands.entity(esail_entity)
        .insert(NewESail {
            origin: PositionVector::new(
                spacecraft_parameters.esail_origin.x(),
                quantities::Length::new::<length::meter>(0.0), 
                quantities::Length::new::<length::meter>(0.0)
            ),
            undeployed_elements:    Vec::new(),
            deployed_elements:      deployed_elements,
            }
        );

    println!("E-sail spawned");
}
