use bevy::prelude::*;
use uom::si::f64 as quantities;
use uom::si::electric_potential::volt;
use uom::lib::marker::PhantomData;
use crate::{ components, resources };

const BODY_RADIUS:      f64 = 0.1;  // meters
const BODY_MASS:        quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 10.0};  // You sure these are in kg?
const ENDMASS_MASS:     quantities::Mass = quantities::Mass {dimension: PhantomData, units: PhantomData, value: 0.05};

const ARROW_LENGTH:     f32 = 100.0;  // pixels?    Move to f64 for consistency?
const X_FIRST_ELEMENT:  f64 = 0.1;  // meters (more like pixels?)

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_cubesat)
            .add_startup_system(Self::spawn_esail)
            .add_startup_system(Self::spawn_axes)
            .add_startup_system(Self::spawn_center_mass)
            ;
    }
}

impl SpawnerPlugin {

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

        commands.entity(cubesat_entity)
            .insert(components::SatelliteBody)
            .insert(components::Mass(BODY_MASS))
            ;
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

            // The SAIL_ELEMENT_MASS thing is not being used. Define it here from the spacecraft parameters method and then use it in physics.rs

            // Endmass has different mass and size
            let (mass, radius) = if number == number_of_elements.value as i32 - 1 {
                (ENDMASS_MASS, 10.0)
            } else {
                //(SAIL_ELEMENT_MASS, 5.0)
                (spacecraft_parameters.segment_mass(), 5.0)
            };

            let segment_length_pixels = spacecraft_parameters.segment_length().value as f32 * simulation_parameters.pixels_per_meter as f32;

            let element = spawn_esail_element(&mut commands, &mut meshes, &mut materials, segment_length_pixels, x, 0.0, 0.0, radius, mass, is_deployed);

            element_vector.push(element);

        }

        // Creating ESail entity and storing the elements inside.
        commands.spawn_empty()
            .insert(components::ESail{elements: element_vector});

    }

    fn spawn_axes (
        mut commands:   Commands,
        mut meshes:     ResMut<Assets<Mesh>>,
        mut materials:  ResMut<Assets<StandardMaterial>>,
        ) {

        // Maybe I could make the axes an invisible entity, and the arrows children entities of that
        // one. Then I could rotate the parent and the children would follow.

        let axes_entity = commands.spawn(
            SpatialBundle { visibility: Visibility { is_visible: true }, ..Default::default() })
            .insert(Name::new("Axes"))
            .insert(components::Axes)
            .id(); 

        // X (red)
        let red         = Color::rgb(1.0, 0.0, 0.0);
        let x_direction = Vec3::new(1.0, 0.0, 0.0); 
        let x_rotation  = Quat::from_rotation_z(std::f32::consts::PI / 2.0);
        let x_arrow     = spawn_arrow(&mut commands, &mut meshes, &mut materials, red, ARROW_LENGTH, x_direction, x_rotation);
        commands.entity(axes_entity).push_children(&[x_arrow]);

        // Y (green)
        let green       = Color::rgb(0.0, 1.0, 0.0);
        let y_direction = Vec3::new(0.0, 1.0, 0.0); 
        let y_rotation  = Quat::from_rotation_y(0.0);
        let y_arrow     = spawn_arrow(&mut commands, &mut meshes, &mut materials, green, ARROW_LENGTH, y_direction, y_rotation);
        commands.entity(axes_entity).push_children(&[y_arrow]);

        // Z (blue)
        let blue        = Color::rgb(0.0, 0.0, 1.0);
        let z_direction = Vec3::new(0.0, 0.0, 1.0); 
        let z_rotation  = Quat::from_rotation_x(std::f32::consts::PI / 2.0);
        let z_arrow     = spawn_arrow(&mut commands, &mut meshes, &mut materials, blue, ARROW_LENGTH, z_direction, z_rotation);
        commands.entity(axes_entity).push_children(&[z_arrow]);
    }

    fn spawn_center_mass(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        ){

        let com_entity = commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 10.0, ..default() })),
                material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }).id();

        commands.entity(com_entity).insert(components::CenterOfMass);
    }
}

fn spawn_esail_element(
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    _segment_length_pixels: f32,
    x: f64, y: f64, z: f64, radius: f32, mass: quantities::Mass, is_deployed: bool,
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
        .insert(components::VerletObject{previous_x: x, previous_y: y, previous_z: z, current_x: x, current_y: y, current_z: z, is_deployed: is_deployed})
        .insert(components::ElectricallyCharged{potential: quantities::ElectricPotential::new::<volt>(0.0)})    // This should be a default of the component
        ;

    return sail_element;
}

fn spawn_arrow (
    commands:   &mut Commands,
    meshes:     &mut ResMut<Assets<Mesh>>,
    materials:  &mut ResMut<Assets<StandardMaterial>>,
    color: Color, length: f32, direction: Vec3, rotation: Quat,
    ) -> Entity {

    let origin = direction * length / 2.0;

    let material = StandardMaterial { base_color: color, emissive: color, perceptual_roughness: 1.0, ..default() };

    return commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { 
            radius: 1.0, 
            depth:  length,
            ..default() })),
        material: materials.add(material),
        transform: Transform {
            translation: origin,
            rotation: rotation,
            ..default()
        },
        ..default()
    }).id();

}