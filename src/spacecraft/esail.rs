use bevy::prelude::*;

// Should this file be called tether? And the component too?

use uom::si::*;
use uom::si::f64 as quantities;  

use crate::{ physics };

use physics::force_vector::ForceVector as ForceVector;
use physics::verlet_object::VerletObject as VerletObject;
use physics::position_vector::PositionVector as PositionVector;

// Test
#[derive(Event)]
pub struct SailExtended;

#[derive(Component)]
pub struct ESail {  
    pub origin:     PositionVector, 
    pub elements:   Vec<VerletObject>,  
}

impl ESail {

    pub fn extend_sail (
        &mut self,
        mut sail_event:   EventWriter<SailExtended>,
    ) {

        // Putting the new element in the second position and not the first,
        // because the first is undeployed and it would mess up everything

        if self.elements.len() > 1 {    // FIXME What if the sail has only one
            if let Some(second_element) = self.elements.get(1).cloned() {
                self.elements.insert(1, second_element);
            }
        }

        println!("Number of elements after extension: {}", self.elements.len()); 

        sail_event.send(SailExtended);
    }

    //pub fn verlet_angle (&self, index: usize) -> quantities::Angle {
    pub fn verlet_angle (&self, index: usize) -> 
        (quantities::Angle, PositionVector) {

        //if index <= 1 { return quantities::Angle::new::<angle::radian>(0.0) };
        if index <= 1 { 
            return (
                quantities::Angle::new::<angle::radian>(0.0),
                PositionVector::empty()
            )
        };

        // If the chain is A-B-C, C being the verlet we're interested in, then
        // A-B is the reference_line, and B-C is the verlet_line

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

        // TEST
        let restoring_direction = reference_line - verlet_line;

        //if index == 2 {
        //    println!("restoring_direction: {:?}", restoring_direction);
        //}

        return (angle, restoring_direction);
    }


    // Wait, instead of returning angle and direction, I could return the vector
    // with the correct magnitude and direction instead

    //pub fn restoring_vector (&self, index: usize) -> Option<PositionVector> {

    //    // TODO Basically everything

    //    if index <= 1 { return None };

    //    // I need the point of the line that is closest to the index
    //    // I will follow this: 
    //    // https://stackoverflow.com/questions/5227373/minimal-perpendicular-vector-between-a-point-and-a-line 

    //    // 

    //    return Some(PositionVector::empty());
    //}
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

        let x = spacecraft_parameters.tether_origin.x() + 
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


    commands.entity(esail_entity)
        .insert(
            ESail {
                origin: PositionVector::new(
                    spacecraft_parameters.tether_origin.x(),
                    zero,
                    zero
                ),
                elements: elements,
            }
        )
        ;

    println!("(New) E-sail spawned");
}
