use bevy::prelude::*;

use uom::si::*;
use uom::si::f64 as quantities;  

use crate::{ physics };

use physics::force_vector::ForceVector as ForceVector;
use physics::verlet_object::VerletObject as VerletObject;
use physics::position_vector::PositionVector as PositionVector;


#[derive(Component)]
pub struct ESail {  
    pub origin:     PositionVector, 
    pub elements:   Vec<VerletObject>,  
}

// A method for calculating the angle of a certain verlet? Based on index or
// what? Index is fine I think

impl ESail {

    // This should return not only the angle but the direction of the restoring
    // force, right?
    // TODO return radians
    pub fn verlet_angle (&self, index: usize) -> f64 {  // TODO Use uom quantity

        if index <= 1 { return 0.0 };

        let reference_line = PositionVector::from_a_to_b(
            self.elements[index - 2].current_coordinates.clone(),
            self.elements[index - 1].current_coordinates.clone()
        );

        let verlet_line = PositionVector::from_a_to_b(
            self.elements[index - 1].current_coordinates.clone(),
            self.elements[index].current_coordinates.clone()
        );

        let angle = physics::position_vector::angle_between(
            &reference_line,
            &verlet_line
        );


        return angle;

        // I have an angle_between method with position vector, is it used, is
        // it good?
    }
}

pub fn spawn_esail (
    mut commands:           Commands,
    spacecraft_parameters:  Res<super::SpacecraftParameters>,
) {

    let esail_entity = commands.spawn(
        SpatialBundle { 
            visibility: Visibility::Visible,
            ..Default::default() 
        })
        .insert(Name::new("New E-sail"))
        .id();

    let number_of_elements = spacecraft_parameters.number_of_esail_elements();

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
            ESail {
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
