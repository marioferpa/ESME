use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{ components, resources };

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

        let ball_shape = shapes::Circle {
            radius: 5.0,
            ..shapes::Circle::default() // Editing the transform later.
        };

        let x1: f32 = 20.0 + 1.0 * esail.resting_distance;

        let element1 = commands
            .spawn(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::GRAY, 1.0),
                },
                Transform::from_xyz(x1, 0.0, 0.0),
            ))
            .insert(components::SailElement{ index: 0 })
            .insert(components::Mass(1.0))
            .insert(components::CanMove{previous_x: x1, previous_y: 0.0})
            .id()
        ;

        // Add the entity to the ESail resource
        //esail.elements.push(Some(element1));
        esail.elements.push(element1);

        let x2: f32 = 20.0 + 2.0 * esail.resting_distance;

        let element2 = commands
            .spawn(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::GRAY, 1.0),
                },
                Transform::from_xyz(x2, 0.0, 0.0),
            ))
            .insert(components::SailElement{ index: 1 })
            .insert(components::Mass(1.0))
            .insert(components::CanMove{previous_x: x2, previous_y: 0.0})
            .id()
        ;

        //esail.elements.push(Some(element2));
        esail.elements.push(element2);

        let x3: f32 = 20.0 + 3.0 * esail.resting_distance;

        let element3 = commands
            .spawn(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::GRAY, 1.0),
                },
                Transform::from_xyz(x3, 0.0, 0.0),
            ))
            .insert(components::SailElement{ index: 1 })
            .insert(components::Mass(1.0))
            .insert(components::CanMove{previous_x: x3, previous_y: 0.0})
            .id()
        ;

        //esail.elements.push(Some(element3));
        esail.elements.push(element3);
    }
}
