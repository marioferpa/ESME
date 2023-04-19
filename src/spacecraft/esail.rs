use bevy::prelude::*;

use uom::si::f64 as quantities;
use uom::si::*;
use uom::lib::marker::PhantomData;  // Consts in uom are not very well supported

use crate::{ physics, components };

const ENDMASS_MASS: quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 0.05};

// I may need a test that ensures that only elements with the verlet_query component get added

#[derive(Component, Debug)]
pub struct ESail {
    pub origin:     physics::position_vector::PositionVector, 
    pub elements:   Vec<Entity>,
    pub undeployed_elements:    Vec<Entity>,
    pub deployed_elements:      Vec<Entity>,
}

impl ESail {

    pub fn vector_to_previous_element (
        &self, 
        index: usize, 
        verlet_query: &Query<&mut physics::verlet_object::VerletObject>
        ) -> physics::position_vector::PositionVector {

        let element_position = &verlet_query.get(self.elements[index]).expect("").current_coordinates;

        if index > 0 {
            let preceding_element_position = &verlet_query.get(self.elements[index-1]).expect("Element not found").current_coordinates;
            return element_position.clone() - preceding_element_position.clone();
        } else {
            return physics::position_vector::PositionVector::zero();
        }
    }

    pub fn deploy_esail ( &mut self, amount: usize ) {

        let count = std::cmp::min(amount, self.undeployed_elements.len() - 1);

        for _ in 0..count {
            let entity = self.undeployed_elements.pop().unwrap();
            self.deployed_elements.insert(0, entity);
        }
    }

    pub fn retract_esail (&mut self, amount: usize) {

        let count = std::cmp::min(amount, self.deployed_elements.len());

        for _ in 0..count {
            let entity = self.deployed_elements.remove(0);
            self.undeployed_elements.push(entity);
        }
    }

    // Temporary
    pub fn print_elements (&self) {
        println!("Undeployed elements: {:?}", self.undeployed_elements);
        println!("Deployed elements: {:?}", self.deployed_elements);
    }
}

pub fn click(
    //mut commands: Commands,
    //spacecraft_parameters: Res<super::SpacecraftParameters>,
    mut esail_query: Query<&mut super::esail::ESail>,  
    keyboard: Res<Input<KeyCode>>,
    ) {

    let mut esail = esail_query.single_mut();

    if keyboard.just_pressed(KeyCode::Up) {

        println!("Deploying!");

        esail.deploy_esail(1);

        esail.print_elements();

    }

    if keyboard.just_pressed(KeyCode::Down) {

        println!("Retracting!");

        esail.retract_esail(1);

        esail.print_elements();
    }
}

pub fn spawn_esail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spacecraft_parameters: Res<super::SpacecraftParameters>,
    ) {

    let mut element_vector: Vec<Entity> = Vec::new();

    //let mut undeployed_elements:    Vec<Entity> = Vec::new();
    //let mut deployed_elements:      Vec<Entity> = Vec::new();
    let mut undeployed_elements:    Vec<Entity> = Vec::with_capacity(50);   // Not working, why?
    let mut deployed_elements:      Vec<Entity> = Vec::with_capacity(50);

    let esail_entity = commands.spawn((
        Name::new("E-sail"),
        SpatialBundle{ visibility: Visibility{ is_visible: true }, ..Default::default() }
    )).id();

    let number_of_elements = spacecraft_parameters.number_of_esail_elements();
    println!("Number of elements: {}", number_of_elements);

    // E-sail elements
    for number in 0.. number_of_elements - 1 {

        // Deploy all except the first one
        //let deployment_state = if number == 0 { false } else { true };

        // Don't deploy any (only the endmass)
        let deployment_state = false;

        // Deploy endmass and one more
        //let deployment_state = if number == 8 { true } else { false };

        println!("Element {} spawned, deployment_state: {}", number, deployment_state);
        
        let element = spawn_esail_element(
            &mut commands, &mut meshes, &mut materials, 
            spacecraft_parameters.esail_origin.x(), spacecraft_parameters.segment_mass(), 
            deployment_state);
        element_vector.push(element);
        
        if deployment_state == false { 
            undeployed_elements.push(element); // Is the order correct?
        } else {
            deployed_elements.push(element);
        }
    }

    // Endmass
    println!("Plus one endmass");
    let endmass_element = spawn_endmass(&mut commands, &mut meshes, &mut materials, spacecraft_parameters.esail_origin.x(), ENDMASS_MASS);
    element_vector.push(endmass_element);
    // ??
    deployed_elements.push(endmass_element);

    println!("Undeployed elements: {:?}", undeployed_elements);
    println!("Deployed elements: {:?}", deployed_elements);

    commands.entity(esail_entity)
        .insert(Name::new("E-sail"))
        .insert(ESail{ 
            origin: physics::position_vector::PositionVector::new(
                            spacecraft_parameters.esail_origin.x(),
                            quantities::Length::new::<length::meter>(0.0), 
                            quantities::Length::new::<length::meter>(0.0)),
            elements:   element_vector,     
            undeployed_elements:    undeployed_elements,
            deployed_elements:      deployed_elements,
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
        .insert(components::Mass(mass))
        .insert(physics::verlet_object::VerletObject { 
            previous_coordinates:   physics::position_vector::PositionVector::new(x, zero, zero),
            current_coordinates:    physics::position_vector::PositionVector::new(x, zero, zero),
            is_deployed:            true,
        });

    return endmass;
}


fn spawn_esail_element(
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    x: quantities::Length, mass: quantities::Mass, deployment: bool,
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
        .insert(Name::new("E-sail element")) 
        .insert(components::Mass(mass))
        .insert(physics::verlet_object::VerletObject { 
            previous_coordinates: physics::position_vector::PositionVector::new(x, zero, zero),
            current_coordinates:  physics::position_vector::PositionVector::new(x, zero, zero),
            is_deployed:          deployment,
        })
        .insert(components::ElectricallyCharged{ ..Default::default() })
        ;

    return sail_element;
}

