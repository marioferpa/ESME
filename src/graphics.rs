use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{ components, resources };

const Z_ESAIL:                  f32 = 1.0;   // Will need to change if I move to 3D
const Z_CENTER_MASS:            f32 = 10.0;
const X_FIRST_ELEMENT:          f32 = 15.0;

//const NUMBER_OF_ESAIL_ELEMENTS: i32 = 20;

const BODY_MASS:                f32 = 10.0;
const SAIL_ELEMENT_MASS:        f32 = 0.01;
const ENDMASS_MASS:             f32 = 0.1;  // Didn't we have a better name for this?

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_cubesat)
            .add_startup_system(spawn_esail)
            .add_startup_system(spawn_center_mass)
            .insert_resource(resources::SpacecraftParameters{elements: Vec::new(), resting_distance: 20.0, ..Default::default()});
    }
}

fn spawn_center_mass(
    mut commands: Commands,
    ){

    let center_mass_shape = shapes::Circle {
        radius: 10.0,
        //radius: 0.0,
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
    ) {

    let sat_shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(50.0),
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
    mut craft_parameters: ResMut<resources::SpacecraftParameters>,
    ) {

    for number in 1..=craft_parameters.number_elements {

        let x = X_FIRST_ELEMENT + number as f32 * craft_parameters.resting_distance;

        if number == 1 {
            // First element, not deployed
            spawn_esail_element(x, 0.0, 5.0, SAIL_ELEMENT_MASS, false, &mut commands, &mut craft_parameters);
        } else {
            if number == craft_parameters.number_elements {
                // Last element, is the endmass
                spawn_esail_element(x, 0.0, 10.0, ENDMASS_MASS, true, &mut commands, &mut craft_parameters);
            } else { 
                // Elements in the middle
                spawn_esail_element(x, 0.0, 5.0, SAIL_ELEMENT_MASS, true, &mut commands, &mut craft_parameters);
            }
        }
    }
}

fn spawn_esail_element(
    x: f32, y: f32, radius: f32, mass: f32, is_deployed: bool,
    commands: &mut Commands,
    esail: &mut ResMut<resources::SpacecraftParameters>,
    ) {

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
        .id()
    ;

    // Add the entity to the ESail resource
    esail.elements.push(sail_element);

}
