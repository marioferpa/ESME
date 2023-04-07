use bevy::prelude::*;

use uom::si::f64 as quantities;
use uom::si::*;
use uom::si::length::meter;
use uom::lib::marker::PhantomData;

use crate::{ physics, components, resources };

const ENDMASS_MASS: quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 0.05};

#[derive(Component, Debug)]
pub struct ESail {
    pub origin:     physics::position_vector::PositionVector, 
    pub elements:   Vec<Entity>,
    //Should length go here and not on the spacecraft parameters?
}

impl ESail {

    pub fn vector_to_previous_element (
        &self, index: usize, verlet_query: &Query<&mut physics::verlet_object::VerletObject>) 
        -> physics::position_vector::PositionVector {

        let element_position = &verlet_query.get(self.elements[index]).expect("").current_coordinates;

        let preceding_element_position = 
            if index > 0 {
                &verlet_query.get(self.elements[index-1]).expect("").current_coordinates
            } else {
                &self.origin
            };

        let relative_vector = element_position.clone() - preceding_element_position.clone();
        return relative_vector;
    }

    /// Adds an esail element (near the cubesat)
    fn add_element(&mut self, esail_element: Entity) {
        self.elements.insert(0, esail_element); 
    }
}

pub fn spawn_esail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    simulation_parameters: Res<resources::SimulationParameters>,
    spacecraft_parameters: Res<super::SpacecraftParameters>,
    ) {

    let mut element_vector: Vec<Entity> = Vec::new();

    let esail_origin_x = spacecraft_parameters.esail_origin.x().get::<meter>();

    let esail_entity = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0, ..default() })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                esail_origin_x as f32 * simulation_parameters.pixels_per_meter as f32, 0.0, 0.0),
                visibility: Visibility{ is_visible: true },
            ..default()
        }).id();

    // User defines length of sail and resolution, elements are calculated from those.
    let number_of_elements = spacecraft_parameters.number_of_esail_elements();
    let distance_between_elements = 1.0 / spacecraft_parameters.wire_resolution.value;  // Using get for linear density would get weird

    //for number in 0..= number_of_elements - 1 {

    //    let x = esail_origin_x + ( number as f64 + 1.0 ) * distance_between_elements;

    //    println!("Element {}, x = {} meters", number, x);
    //    
    //    let element = if number == number_of_elements - 1 {
    //        // Endmass
    //        spawn_endmass(&mut commands, &mut meshes, &mut materials, spacecraft_parameters.esail_origin.x(), ENDMASS_MASS)
    //    } else {
    //        // Sail segment
    //        //spawn_esail_element(&mut commands, &mut meshes, &mut materials, x, 0.0, 0.0, 5.0, spacecraft_parameters.segment_mass(), false)
    //        spawn_esail_element(&mut commands, &mut meshes, &mut materials, spacecraft_parameters.esail_origin.x(), spacecraft_parameters.segment_mass())
    //    };

    //    element_vector.push(element);
    //}

    element_vector.push(
        spawn_endmass(&mut commands, &mut meshes, &mut materials, spacecraft_parameters.esail_origin.x(), ENDMASS_MASS)
        );

    commands.entity(esail_entity)
        .insert(Name::new("E-sail"))
        .insert(ESail{ 
            origin: physics::position_vector::PositionVector::new(
                            spacecraft_parameters.esail_origin.x(),
                            quantities::Length::new::<length::meter>(0.0), 
                            quantities::Length::new::<length::meter>(0.0)),
            elements:   element_vector,     
        })
    ;

    println!("E-sail spawned");
}

fn spawn_endmass (
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    x: quantities::Length, mass: quantities::Mass,
    ) -> Entity {

    let endmass = 
        commands.spawn ( 
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 15.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }
        ).id();

    let zero = quantities::Length::new::<length::meter>(0.0);

    commands.entity(endmass)
        .insert(Name::new("Endmass")) 
        .insert(physics::verlet_object::VerletObject { 
            previous_coordinates: physics::position_vector::PositionVector::new(x, zero, zero),
            current_coordinates:  physics::position_vector::PositionVector::new(x, zero, zero),
        });

    return endmass;
}


fn spawn_esail_element(
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    x: quantities::Length, mass: quantities::Mass,
    ) -> Entity {

    let radius = 5.0; // 5.0 what? Apples? Oranges? 

    let sail_element =
        commands.spawn ( 
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: radius, ..default() })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }
        ).id();

    let zero = quantities::Length::new::<length::meter>(0.0);

    commands.entity(sail_element)
        .insert(Name::new("E-sail element"))    // Add index to the name!
        .insert(components::Mass(mass))
        .insert(physics::verlet_object::VerletObject { 
            previous_coordinates: physics::position_vector::PositionVector::new(x, zero, zero),
            current_coordinates:  physics::position_vector::PositionVector::new(x, zero, zero),
        })
        .insert(components::ElectricallyCharged{ ..Default::default() })
        ;

    return sail_element;
}

