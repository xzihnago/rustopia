use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

use rustopia_settings::Settings;

use crate::player::Player;

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, player_moving)
            .add_systems(Update, camera_rotation);
    }
}

fn startup(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn((
        Player::default(),
        Transform::from_xyz(5., 5., 2.).looking_at(Vec3::ZERO, Vec3::Z),
        Camera3d {
            screen_space_specular_transmission_steps: settings.graphic.specular_transmission.step,
            screen_space_specular_transmission_quality: settings
                .graphic
                .specular_transmission
                .quality,
            ..default()
        },
    ));
}

pub fn player_moving(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut camera: Query<&mut Transform, With<Player>>,
) {
    let mut camera = camera.single_mut();

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(settings.keybind.move_forward) {
        direction += camera.forward().xyz();
    }
    if keyboard.pressed(settings.keybind.move_backward) {
        direction += camera.back().xyz();
    }

    if keyboard.pressed(settings.keybind.move_right) {
        direction += camera.right().xyz();
    }
    if keyboard.pressed(settings.keybind.move_left) {
        direction += camera.left().xyz();
    }

    if keyboard.pressed(settings.keybind.move_up) {
        direction += camera.up().xyz();
    }
    if keyboard.pressed(settings.keybind.move_down) {
        direction += camera.down().xyz();
    }

    if keyboard.pressed(settings.keybind.move_up_axis) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(settings.keybind.move_down_axis) {
        direction += Vec3::NEG_Z;
    }

    camera.translation += 10. * time.delta_secs() * direction;
}

pub fn camera_rotation(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut mouse: EventReader<MouseMotion>,
    window: Query<&Window>,
    mut camera: Query<&mut Transform, With<Player>>,
) {
    if let Ok(window) = window.get_single() {
        if let CursorGrabMode::Locked = window.cursor_options.grab_mode {
            let mut camera = camera.single_mut();

            let delta_time = time.delta_secs();
            let delta_angle = settings.control.mouse_sensitivity * delta_time;

            // Pitch and Yaw
            mouse.read().for_each(|motion| {
                camera.rotate_local_x(delta_angle * -motion.delta.y);
                camera.rotate_z(delta_angle * -motion.delta.x);
            });

            // Roll
            let axis_roll = camera.forward();
            if keyboard.pressed(settings.keybind.roll_counterclockwise) {
                camera.rotate_axis(axis_roll, -delta_time);
            }
            if keyboard.pressed(settings.keybind.roll_clockwise) {
                camera.rotate_axis(axis_roll, delta_time);
            }
        }
    }
}
