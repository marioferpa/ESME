use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::si::f64 as quantities;

use uom::lib::marker::PhantomData;
use uom::si::electric_potential::volt;

use crate::{ components, resources };

const X_FIRST_ELEMENT:          f64 = 0.1;  // meters (more like pixels?)

const Z_ESAIL:                  f64 = 1.0;  // Will need to change if I move to 3D
const Z_CENTER_MASS:            f64 = 10.0;

const BODY_MASS:         quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 10.0};  // You sure these are in kg?
const SAIL_ELEMENT_MASS: quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 0.01}; // Isn't this defined somewhere else from aluminiums density?
const ENDMASS_MASS:      quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 0.05};

const BODY_RADIUS:      f64 = 0.1;  // meters

const ARROW_LENGTH:     f32 = 100.0;  // pixels?

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_light)
            .add_startup_system(spawn_cubesat)
            .add_startup_system(spawn_esail)
            .add_startup_system(spawn_center_mass)
            .add_startup_system(spawn_axes)
            .add_system(Self::gizmo_visibility)
            ;
    }
}

impl GraphicsPlugin {
    // Maybe this could go together with the camera
    fn spawn_light( mut commands: Commands) {
        commands.spawn(DirectionalLightBundle {
            ..default()
        });
    }

    fn gizmo_visibility (
        mut com_query:          Query<&mut Visibility, With<components::CenterOfMass>>, 
        //mut axes_query:         Query<&mut Visibility, With<components::CenterOfMass>>,   // Need the axes to be an entity now
        simulation_parameters:  Res<resources::SimulationParameters>,
        ) {

        let mut com_visibility = com_query.single_mut();
        com_visibility.is_visible = simulation_parameters.com_visibility;
    }
}


fn spawn_axes (
    mut commands:   Commands,
    mut meshes:     ResMut<Assets<Mesh>>,
    mut materials:  ResMut<Assets<StandardMaterial>>,
    ) {

    // Maybe I could make the axes an invisible entity, and the arrows children entities of that
    // one. Then I could rotate the parent and the children would follow.

    let red     = Color::rgb(1.0, 0.0, 0.0);
    let green   = Color::rgb(0.0, 1.0, 0.0);
    let blue    = Color::rgb(0.0, 0.0, 1.0);

    let x_direction = Vec3::new(1.0, 0.0, 0.0); 
    let y_direction = Vec3::new(0.0, 1.0, 0.0); 
    let z_direction = Vec3::new(0.0, 0.0, 1.0); 

    let x_rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0);
    let y_rotation = Quat::from_rotation_y(0.0);
    let z_rotation = Quat::from_rotation_x(std::f32::consts::PI / 2.0);

    // X (red)
    spawn_arrow(&mut commands, &mut meshes, &mut materials, red, ARROW_LENGTH, x_direction, x_rotation);

    // Y (green)
    spawn_arrow(&mut commands, &mut meshes, &mut materials, green, ARROW_LENGTH, y_direction, y_rotation);

    // Z (blue)
    spawn_arrow(&mut commands, &mut meshes, &mut materials, blue, ARROW_LENGTH, z_direction, z_rotation);
}

fn spawn_arrow (
    mut commands:   &mut Commands,
    mut meshes:     &mut ResMut<Assets<Mesh>>,
    mut materials:  &mut ResMut<Assets<StandardMaterial>>,
    color: Color, length: f32, direction: Vec3, rotation: Quat,
    ) -> Entity {

    let origin = direction * length / 2.0;

    let material = StandardMaterial { base_color: color, emissive: color, perceptual_roughness: 1.0, ..default() };

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { 
            radius: 1.0, 
            depth:  length,
            ..default() })),
        material: materials.add(material),
        transform: Transform {
            translation: origin,
            //rotation: Quat::from_axis_angle(direction, std::f32::consts::PI / 2.0), // Angle in radians!!
            rotation: rotation,
            ..default()
        },
        ..default()
    }).id()

}

fn spawn_center_mass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    simulation_parameters: Res<resources::SimulationParameters>,
    ){

    let center_mass_shape = shapes::Circle {
        radius: 10.0,
        ..shapes::Circle::default() // Editing the transform later.
    };

    let com_entity = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 10.0, ..default() })),
            material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }).id();

    commands.entity(com_entity).insert(components::CenterOfMass);
}

fn spawn_cubesat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    simulation_parameters: Res<resources::SimulationParameters>,
    ) {

    let cubesat_size = BODY_RADIUS * simulation_parameters.pixels_per_meter as f64 / 0.707;

    let cubesat_entity = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: cubesat_size as f32})),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }).id();

    commands.entity(cubesat_entity).insert(components::Mass(BODY_MASS));
}

fn spawn_esail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    simulation_parameters: Res<resources::SimulationParameters>,
    spacecraft_parameters: Res<resources::SpacecraftParameters>,
    ) {

    let mut element_vector: Vec<Entity> = Vec::new();

    // User defines length of sail and resolution, elements are calculated from those.
    let number_of_elements = spacecraft_parameters.wire_length * spacecraft_parameters.wire_resolution;
    let pixels_between_elements = (1.0 / spacecraft_parameters.wire_resolution.value) * simulation_parameters.pixels_per_meter as f64;   // Pixels

    // x is in pixels here, I think that is correct.

    for number in 0..= number_of_elements.value as i32 - 1 {
 
        let x = X_FIRST_ELEMENT * simulation_parameters.pixels_per_meter as f64 + number as f64 * pixels_between_elements;

        // The first element stays undeployed and is unaffected by forces
        let is_deployed = match number {
            0 => false,
            _ => true,
        };

        // Endmass has different mass and size
        let (mass, radius) = if number == number_of_elements.value as i32 - 1 {
            (ENDMASS_MASS, 10.0)
        } else {
            (SAIL_ELEMENT_MASS, 5.0)
        };

        let segment_length_pixels = spacecraft_parameters.segment_length().value as f32 * simulation_parameters.pixels_per_meter as f32;

        //let element = spawn_esail_element(&mut commands, segment_length_pixels, x, 0.0, radius, mass, is_deployed);

        let element = spawn_3d_esail_element(&mut commands, &mut meshes, &mut materials, segment_length_pixels, x, 0.0, radius, mass, is_deployed);

        element_vector.push(element);

    }

    // Creating ESail entity and storing the elements inside.
    commands.spawn_empty()
        .insert(components::ESail{elements: element_vector});

}

fn spawn_3d_esail_element(
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    _segment_length_pixels: f32,
    x: f64, y: f64, radius: f32, mass: quantities::Mass, is_deployed: bool,
    ) -> Entity {

    let sail_element = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: radius, ..default() })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(x as f32, y as f32, 0.0),
            ..default()
        }).id();

    commands.entity(sail_element)
        .insert(components::SailElement{is_deployed: is_deployed})
        .insert(components::Mass(mass))
        .insert(components::VerletObject{previous_x: x, previous_y: y, current_x: x, current_y: y, is_deployed: is_deployed})
        .insert(components::ElectricallyCharged{potential: quantities::ElectricPotential::new::<volt>(0.0)})    // This should be a default of the component
        ;

    return sail_element;
}

fn spawn_esail_element(
    commands: &mut Commands,
    _segment_length_pixels: f32,
    x: f64, y: f64, radius: f32, mass: quantities::Mass, is_deployed: bool,
    ) -> Entity {

    let esail_element_shape = shapes::Circle {
        radius: radius,
        ..shapes::Circle::default() // Editing the transform later.
    };

    // Maybe it's easier to draw lines between the points, instead of turning circles into rectangles.
    // A bit unrealistic since the rectangles that I will be operating on will be centered around the circles, but who cares.
    
    let sail_element = commands
        .spawn(GeometryBuilder::build_as(
            &esail_element_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::new(Color::GRAY, 1.0),
            },
            Transform::from_xyz(x as f32, y as f32, Z_ESAIL as f32),
        ))
        .id()
    ;

    commands.entity(sail_element)
        .insert(components::SailElement{is_deployed: is_deployed})
        .insert(components::Mass(mass))
        .insert(components::VerletObject{previous_x: x, previous_y: y, current_x: x, current_y: y, is_deployed: is_deployed})
        .insert(components::ElectricallyCharged{potential: quantities::ElectricPotential::new::<volt>(0.0)})    // This should be a default of the component
        ;

    return sail_element;
}
