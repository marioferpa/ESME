use bevy::prelude::*;
use bevy::math::DVec3;  // Vec3 with f64 values
use uom::si::f64 as quantities;
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
            .add_startup_system_set(
                SystemSet::new()
                    .label("spawn_elements")
                    .with_system(Self::spawn_cubesat)
                    .with_system(Self::spawn_esail)
                    .with_system(Self::spawn_axes)
                    .with_system(Self::spawn_center_mass)
            )
        ;
    }
}

impl SpawnerPlugin {

    fn spawn_cubesat(
        mut commands:           Commands,
        mut meshes:             ResMut<Assets<Mesh>>,
        mut materials:          ResMut<Assets<StandardMaterial>>,
        simulation_parameters:  Res<resources::SimulationParameters>,
        ) {

        let cubesat_size = BODY_RADIUS * simulation_parameters.pixels_per_meter as f64 / 0.707;

        let cubesat_entity = commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: cubesat_size as f32})),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            }).id();

        commands.entity(cubesat_entity)
            .insert(Name::new("Satellite body"))
            .insert(components::SatelliteBody)
            .insert(components::Mass(BODY_MASS))
            ;

        println!("Cubesat spawned");
    }

    fn spawn_esail(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        simulation_parameters: Res<resources::SimulationParameters>,
        spacecraft_parameters: Res<resources::SpacecraftParameters>,
        ) {

        let mut element_vector: Vec<Entity> = Vec::new();

        //let esail_entity = commands.spawn((
        //    Name::new("E-sail"),
        //    // Should move this to the side of the cubesat at some point
        //    SpatialBundle{ visibility: Visibility{ is_visible: true }, ..Default::default() }
        //)).id();

        // Spawning a mesh (red cube) on the E-sail position for easier debugging

        let esail_entity = commands.spawn(PbrBundle {
                //mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 10.0, ..default() })),
                mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0, ..default() })),
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform: Transform::from_xyz(
                    X_FIRST_ELEMENT as f32 * simulation_parameters.pixels_per_meter as f32, 0.0, 0.0),
                    visibility: Visibility{ is_visible: true },
                ..default()
            }).id();

        // User defines length of sail and resolution, elements are calculated from those.
        let number_of_elements = spacecraft_parameters.wire_length * spacecraft_parameters.wire_resolution;
        let pixels_between_elements = (1.0 / spacecraft_parameters.wire_resolution.value) * simulation_parameters.pixels_per_meter as f64;   // Pixels

        for number in 0..= number_of_elements.value as i32 - 1 {
     
            let x = X_FIRST_ELEMENT * simulation_parameters.pixels_per_meter as f64 + (number + 1) as f64 * pixels_between_elements;
            //let x = X_FIRST_ELEMENT * simulation_parameters.pixels_per_meter as f64 + number as f64 * pixels_between_elements;
            
            let element = if number == number_of_elements.value as i32 - 1 {
                // Endmass
                spawn_esail_element(&mut commands, &mut meshes, &mut materials, x, 0.0, 0.0, 10.0, ENDMASS_MASS, true)
            } else {
                // Sail segment
                spawn_esail_element(&mut commands, &mut meshes, &mut materials, x, 0.0, 0.0, 5.0, spacecraft_parameters.segment_mass(), false)
            };

            element_vector.push(element);
        }

        commands.entity(esail_entity)
            .insert(components::ESail{ 
                //origin:     (X_FIRST_ELEMENT * simulation_parameters.pixels_per_meter as f64, 0.0, 0.0),
                origin:     DVec3::new(X_FIRST_ELEMENT * simulation_parameters.pixels_per_meter as f64, 0.0, 0.0),
                elements:   element_vector,     
            });

        println!("E-sail spawned");


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
    x: f64, y: f64, z: f64, radius: f32, mass: quantities::Mass,
    is_endmass: bool,
    ) -> Entity {

    let sail_element = if is_endmass {
        commands.spawn (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 15.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..default()
            }
        ).id()

    } else {
        commands.spawn ( 
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: radius, ..default() })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..default()
            }
        ).id()
    };

    commands.entity(sail_element)
        .insert(Name::new("E-sail element"))    // Add index to the name!
        .insert(components::Mass(mass))
        .insert(components::VerletObject{previous_coordinates: DVec3::new(x, y, z), current_coordinates: DVec3::new(x, y, z), is_deployed: true})
        ;

    if !is_endmass {
        commands.entity(sail_element).insert(components::ElectricallyCharged{ ..Default::default() });
    }

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
