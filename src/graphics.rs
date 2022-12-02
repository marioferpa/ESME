use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{ components };

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_shapes);
    }
}

impl GraphicsPlugin {

    fn spawn_shapes(mut commands: Commands) {

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

        commands
            .spawn(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::GRAY, 1.0),
                },
                Transform::from_xyz(50.0, 0.0, 0.0),
            ))
            .insert(components::SailElement)
            .insert(components::Mass(1.0))
            .insert(components::CanMove{previous_x: 50.0, previous_y: 0.0})
            ;

        commands
            .spawn(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::GRAY, 1.0),
                },
                Transform::from_xyz(70.0, 0.0, 0.0),
            ))
            .insert(components::SailElement)
            .insert(components::Mass(1.0))
            .insert(components::CanMove{previous_x: 50.0, previous_y: 0.0})
            ;

        commands
            .spawn(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::GRAY, 1.0),
                },
                Transform::from_xyz(90.0, 0.0, 0.0),
            ))
            .insert(components::SailElement)
            .insert(components::Mass(1.0))
            .insert(components::CanMove{previous_x: 50.0, previous_y: 0.0})
            ;

    }
}
