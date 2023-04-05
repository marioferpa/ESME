use bevy::prelude::*;

use uom::si::f64 as quantities;
use uom::si::*;
use uom::si::length::meter;
use uom::lib::marker::PhantomData;

use crate::{ physics, components, resources };

const BODY_MASS:        quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 10.0};  // You sure these are in kg?
const ENDMASS_MASS:     quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 0.05};

#[derive(Component, Debug)]
pub struct ESail {
    pub origin:     physics::position_vector::PositionVector, 
    pub elements:   Vec<Entity>,    // Why's this Vec<Entity> and not Vec<VerletObject>?
}

#[derive(Component, Debug)]
pub struct ESail2 {
    pub origin: physics::position_vector::PositionVector,
} 

impl ESail {

    pub fn distance_between_elements (
        &self, index: usize, verlet_query: &Query<&mut physics::verlet_object::VerletObject>) 
        -> physics::position_vector::PositionVector {

        let element_position = &verlet_query.get(self.elements[index]).expect("").current_coordinates;

        let preceding_element_position = 
            if index > 0 {
                &verlet_query.get(self.elements[index-1]).expect("").current_coordinates
            } else {
                //&self.origin_new
                &self.origin
            };

        let distance = element_position.clone() - preceding_element_position.clone();
        return distance;
    }
}

pub fn spawn_second_esail (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    simulation_parameters: Res<resources::SimulationParameters>,
    ) {

    //let x_position = (super::body::BODY_X * simulation_parameters.pixels_per_meter as f64) / 2.0;
    let x_position = 0.5 * simulation_parameters.pixels_per_meter as f64; 

    let esail_entity = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 10.0, ..default() })),
            //mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0, ..default() })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                //50.0, 0.0, 0.0),
                x_position as f32, 0.0, 0.0),
                visibility: Visibility{ is_visible: true },
            ..default()
        }).id();

    commands.entity(esail_entity)
        .insert(Name::new("New E-sail"))
        .insert(ESail2 {
            origin: physics::position_vector::PositionVector::new(
                            //quantities::Length::new::<length::meter>(super::body::BODY_X / 2.0), // Not multiplied by pixels per meter??
                            quantities::Length::new::<length::meter>(x_position), 
                            quantities::Length::new::<length::meter>(0.0), 
                            quantities::Length::new::<length::meter>(0.0)),
            });
}

pub fn spawn_esail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    simulation_parameters: Res<resources::SimulationParameters>,
    spacecraft_parameters: Res<super::SpacecraftParameters>,
    ) {

    let mut element_vector: Vec<Entity> = Vec::new();

    //let esail_entity = commands.spawn((
    //    Name::new("E-sail"),
    //    // Should move this to the side of the cubesat at some point
    //    SpatialBundle{ visibility: Visibility{ is_visible: true }, ..Default::default() }
    //)).id();

    let esail_origin_x = spacecraft_parameters.esail_origin.x().get::<meter>();

    // Red cube at the origin of the sail, for debugging
    let esail_entity = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0, ..default() })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                esail_origin_x as f32 * simulation_parameters.pixels_per_meter as f32, 0.0, 0.0),
                visibility: Visibility{ is_visible: true },
            ..default()
        }).id();

    // User defines length of sail and resolution, elements are calculated from those.
    let number_of_elements = spacecraft_parameters.wire_length * spacecraft_parameters.wire_resolution; // Should be a method of sp parameters
    let distance_between_elements = 1.0 / spacecraft_parameters.wire_resolution.value;  // Using get for linear density would get weird

    println!("Number of elements: {:?}", number_of_elements);

    for number in 0..= number_of_elements.value as i32 - 1 {

        let x = esail_origin_x + ( number as f64 + 1.0 ) * distance_between_elements;

        println!("Element {}, x = {} meters", number, x);
        
        let element = if number == number_of_elements.value as i32 - 1 {
            // Endmass
            spawn_esail_element(&mut commands, &mut meshes, &mut materials, x, 0.0, 0.0, 10.0, ENDMASS_MASS, true)
        } else {
            // Sail segment
            spawn_esail_element(&mut commands, &mut meshes, &mut materials, x, 0.0, 0.0, 5.0, spacecraft_parameters.segment_mass(), false)
        };

        element_vector.push(element);
    }

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

fn spawn_esail_element(
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    x: f64, y: f64, z: f64, radius: f32, mass: quantities::Mass,
    is_endmass: bool,
    ) -> Entity {

    let sail_element = if is_endmass {
        commands.spawn (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 15.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..default()
            }
        ).id()

    } else {
        commands.spawn ( 
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: radius, ..default() })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..default()
            }
        ).id()
    };

    let x_si = quantities::Length::new::<meter>(x);
    let y_si = quantities::Length::new::<meter>(y);
    let z_si = quantities::Length::new::<meter>(z);

    commands.entity(sail_element)
        .insert(Name::new("E-sail element"))    // Add index to the name!
        .insert(components::Mass(mass))
        //.insert(components::VerletObject{previous_coordinates: DVec3::new(x, y, z), current_coordinates: DVec3::new(x, y, z)})
        .insert(physics::verlet_object::VerletObject { 
            previous_coordinates: physics::position_vector::PositionVector::new(x_si, y_si, z_si),
            current_coordinates:  physics::position_vector::PositionVector::new(x_si, y_si, z_si),
        })
        ;

    if !is_endmass {
        commands.entity(sail_element).insert(components::ElectricallyCharged{ ..Default::default() });
    }

    return sail_element;
}

