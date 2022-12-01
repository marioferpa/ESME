use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod components;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })   // What's this
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(camera_system)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {

    let sat_shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(50.0),
        ..shapes::RegularPolygon::default()
    };

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

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
