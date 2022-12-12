use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{ components, resources };

const Z_ESAIL: f32 = 1.0;   // Will need to change if I move to 3D

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_shapes)
            .insert_resource(resources::ESail{elements: Vec::new(), resting_distance: 20.0});
    }
}

impl GraphicsPlugin {


    fn spawn_shapes(
        mut commands: Commands,
        mut esail: ResMut<resources::ESail>,
        ) {

        // Satellite body

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
            .insert(components::Mass(10.0));

        // Sail elements

        let x1: f32 = 20.0 + 1.0 * esail.resting_distance;

        spawn_esail_element(x1, 0.0, &mut commands, &mut esail);

        let x2: f32 = 20.0 + 2.0 * esail.resting_distance;

        spawn_esail_element(x2, 0.0, &mut commands, &mut esail);

        let x3: f32 = 20.0 + 3.0 * esail.resting_distance;

        spawn_esail_element(x3, 0.0, &mut commands, &mut esail);

    }
}

fn spawn_esail_element(
    x: f32, y: f32,
    commands: &mut Commands,
    esail: &mut ResMut<resources::ESail>,
    ) {

    let esail_element_shape = shapes::Circle {
        radius: 5.0,
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
        .insert(components::SailElement{})
        .insert(components::CanMove{previous_x: x, previous_y: y})
        .id()
    ;

    // Add the entity to the ESail resource
    esail.elements.push(sail_element);

}
