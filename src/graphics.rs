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

        let sat_shape = shapes::RegularPolygon {
            sides: 4,
            feature: shapes::RegularPolygonFeature::Radius(50.0),
            ..shapes::RegularPolygon::default()
        };

        // RegularPolygon also includes a "center", but it's not set.
        // Do the same with the ball, and manipulate it later.
        let ball_shape = shapes::Circle {
            radius: 5.0,
            center: Vec2::new(70.0, 0.0),
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

        commands
            .spawn(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::GRAY, 1.0),
                },
                Transform::default(),
            ));
    }
}
