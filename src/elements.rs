use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{ components, resources };

const X_FIRST_ELEMENT:          f32 = 0.1;  // meters (?)

const Z_ESAIL:                  f32 = 1.0;  // Will need to change if I move to 3D
const Z_CENTER_MASS:            f32 = 10.0;

const BODY_MASS:                f32 = 10.0;
const SAIL_ELEMENT_MASS:        f32 = 0.01;
const ENDMASS_MASS:             f32 = 0.1;

const BODY_RADIUS:              f32 = 0.1;  // meters

pub struct ElementsPlugin;

impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_cubesat)
            .add_startup_system(spawn_esail)
            .add_startup_system(spawn_center_mass)
            ;
    }
}

fn spawn_center_mass(
    mut commands: Commands,
    ){

    let center_mass_shape = shapes::Circle {
        radius: 10.0,
        ..shapes::Circle::default() // Editing the transform later.
    };

    commands.spawn(GeometryBuilder::build_as(
            &center_mass_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::from_xyz(0.0, 0.0, Z_CENTER_MASS),
        ))
        .insert(components::CenterOfMass)
        ;
}

fn spawn_cubesat(
    mut commands: Commands,
    simulation_parameters: Res<resources::SimulationParameters>,
    ) {

    let sat_shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(BODY_RADIUS * simulation_parameters.pixels_per_meter as f32 / 0.707),
        ..shapes::RegularPolygon::default()
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &sat_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GRAY),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::default(),
        ))
        .insert(components::Mass(BODY_MASS));
}

fn spawn_esail(
    mut commands: Commands,
    simulation_parameters: Res<resources::SimulationParameters>,
    spacecraft_parameters: ResMut<resources::SpacecraftParameters>,
    ) {

    let mut element_vector: Vec<Entity> = Vec::new();

    // User defines length of sail and resolution, elements are calculated from those.
    let number_of_elements = (spacecraft_parameters.wire_length_m * spacecraft_parameters.wire_resolution) as i32;
    let distance_between_elements = (1.0 / spacecraft_parameters.wire_resolution) * simulation_parameters.pixels_per_meter as f32;

    for number in 0..=number_of_elements-1 {

        let x = X_FIRST_ELEMENT * simulation_parameters.pixels_per_meter as f32 + number as f32 * distance_between_elements;
        //println!("x: {} pixels", x);

        // The first element stays undeployed and is unaffected by forces
        let is_deployed = match number {
            0 => false,
            _ => true,
        };

        // Endmass has different mass and size
        let (mass, radius) = if number == number_of_elements - 1 {
            (ENDMASS_MASS, 10.0)
        } else {
            (SAIL_ELEMENT_MASS, 5.0)
        };

        //println!("Mass: {}, radius: {}", mass, radius);

        let element = spawn_esail_element(&mut commands, x, 0.0, radius, mass, is_deployed);
        element_vector.push(element);

    }

    // Creating ESail entity and storing the elements inside.
    commands.spawn_empty()
        .insert(components::ESail{elements: element_vector, resting_distance: distance_between_elements});

}

fn spawn_esail_element(
    commands: &mut Commands,
    x: f32, y: f32, radius: f32, mass: f32, is_deployed: bool,
    ) -> Entity {

    let esail_element_shape = shapes::Circle {
        radius: radius,
        ..shapes::Circle::default() // Editing the transform later.
    };

    let sail_element = commands
        .spawn(GeometryBuilder::build_as(
            &esail_element_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::new(Color::GRAY, 1.0),
            },
            Transform::from_xyz(x, y, Z_ESAIL),
        ))
        .insert(components::SailElement) 
        .insert(components::Mass(mass))
        .insert(components::VerletObject{previous_x: x, previous_y: y, current_x: x, current_y: y, is_deployed: is_deployed})
        // NEW!!
        .insert(components::ElectricallyCharged{potential: 0.0})
        .id()
    ;

    return sail_element;
}
