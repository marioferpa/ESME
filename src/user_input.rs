use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;

use bevy::window::PrimaryWindow;

use crate::{ components, graphics, spacecraft };

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, (
                    pan_orbit_camera,
                    //keyboard_input
                )
            )  
        ;
    }
}



pub fn keyboard_input (
    mut esail_query: Query<&mut spacecraft::esail::ESail>,  
    keyboard: Res<Input<KeyCode>>,
) {

    let mut esail = esail_query.single_mut();

    if keyboard.just_pressed(KeyCode::Up) {

        println!("Deploying!");

        esail.deploy_esail(1);

        esail.print_elements();

    }

    if keyboard.just_pressed(KeyCode::Down) {

        println!("Retracting!");

        esail.retract_esail(1);

        esail.print_elements();
    }
}



fn pan_orbit_camera(
    window_query:       Query<&Window, With<PrimaryWindow>>,
    mut ev_motion:      EventReader<MouseMotion>,
    mut ev_scroll:      EventReader<MouseWheel>,
    input_mouse:        Res<Input<MouseButton>>,
    mut camera_query:   Query<(&mut graphics::camera::Camera, &mut Transform, &Projection)>,
) {

    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in camera_query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            //let window = lights_and_camera::get_primary_window_size(&windows);
            let window = graphics::get_primary_window_size(&window_query);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            //let window = lights_and_camera::get_primary_window_size(&windows);
            let window = graphics::get_primary_window_size(&window_query);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}
