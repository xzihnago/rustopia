use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;

use rustopia_settings::Settings;

use crate::{Player, PlayerGroundSensor};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, startup);
        // .add_systems(PreUpdate, mark_grounded)
        // .add_systems(Update, player_moving);
        // .add_systems(Update, camera_rotation);
    }
}

fn startup(mut commands: Commands, settings: Res<Settings>) {
    // Debug camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10., 10., 10.),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // Player
    // commands
    //     .spawn((
    //         Player::default(),
    //         SpatialBundle::from(Transform::from_xyz(5., 5., 0.)),
    //         // Rigid Body
    //         RigidBody::Dynamic,
    //         Velocity::default(),
    //         LockedAxes::ROTATION_LOCKED,
    //         Ccd::enabled(),
    //         // Character controller
    //         KinematicCharacterController {
    //             up: Vec3::Z,
    //             ..default()
    //         },
    //     ))
    //     .with_children(|parent| {
    //         // Collider
    //         parent.spawn((
    //             Transform::from_xyz(0., 0., 0.8).looking_to(Vec3::Y, Vec3::Z),
    //             Collider::round_cylinder(0.7, 0.2, 0.1),
    //             ColliderMassProperties::Mass(60.),
    //             Friction::new(1.),
    //         ));

    //         // Ground sensor
    //         parent.spawn((
    //             PlayerGroundSensor,
    //             Transform::from_xyz(0., 0., 0.1).looking_to(Vec3::Y, Vec3::Z),
    //             Collider::round_cylinder(0.02, 0.2, 0.09),
    //             ActiveEvents::COLLISION_EVENTS,
    //             // Sensor,
    //         ));

    //         // Camera
    //         parent.spawn((
    //             Camera3dBundle {
    //                 transform: Transform::from_xyz(0., 0., 1.5).looking_to(Vec3::Y, Vec3::Z),
    //                 camera_3d: Camera3d {
    //                     screen_space_specular_transmission_steps: settings
    //                         .graphic
    //                         .specular_transmission
    //                         .step,
    //                     screen_space_specular_transmission_quality: settings
    //                         .graphic
    //                         .specular_transmission
    //                         .quality,
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //             PanOrbitCamera {
    //                 base_transform: Transform::from_xyz(0., 0., 1.5).looking_to(Vec3::Y, Vec3::Z),
    //                 ..default()
    //             },
    //         ));
    //     });
}

fn mark_grounded(
    mut collisions: EventReader<CollisionEvent>,
    sensor: Query<Entity, With<PlayerGroundSensor>>,
    mut player: Query<&mut Player>,
) {
    let sensor = sensor.single();
    let mut player = player.single_mut();

    collisions.read().for_each(|event| match event {
        CollisionEvent::Started(entity, ..) if entity == &sensor => {
            player.is_grounded = true;
        }
        CollisionEvent::Stopped(entity, ..) if entity == &sensor => {
            player.is_grounded = false;
        }
        _ => {}
    })
}

pub fn player_moving(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    camera: Query<&Transform, With<Camera>>,
    mut player: Query<(&Player, &mut Velocity)>,
) {
    let (player, mut velocity) = player.single_mut();
    let camera = camera.single();

    let delta_time = time.delta_secs();

    if player.free_move {
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

        velocity.linvel += delta_time * direction;
    } else if player.is_grounded {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(settings.keybind.move_forward) {
            direction += match camera.forward().xyz() {
                v if v == Vec3::Z => camera.down().xyz(),
                v if v == Vec3::NEG_Z => camera.up().xyz(),
                v => v.xy().normalize().extend(0.),
            };
        }
        if keyboard.pressed(settings.keybind.move_backward) {
            direction += match camera.back().xyz() {
                v if v == Vec3::Z => camera.down().xyz(),
                v if v == Vec3::NEG_Z => camera.up().xyz(),
                v => v.xy().normalize().extend(0.),
            };
        }

        if keyboard.pressed(settings.keybind.move_right) {
            direction += camera.right().xyz();
        }
        if keyboard.pressed(settings.keybind.move_left) {
            direction += camera.left().xyz();
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            velocity.linvel = velocity.linvel.lerp(
                match keyboard.pressed(settings.keybind.move_sprint) {
                    b if b => 2. * player.move_speed,
                    _ => player.move_speed,
                } * delta_time
                    * direction,
                (delta_time * 10.).min(1.),
            );
        }

        if keyboard.pressed(settings.keybind.move_up) {
            velocity.linvel += 5. * Vec3::Z;
        }
    }
}

pub fn camera_rotation(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut mouse: EventReader<MouseMotion>,
    windows: Query<&Window>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut player: Query<(&mut Transform, &Player), Without<Camera>>,
) {
    if let CursorGrabMode::Locked = windows.single().cursor_options.grab_mode {
        let mut camera = camera.single_mut();
        let (mut player_transform, player) = player.single_mut();

        let delta_time = time.delta_secs();
        let delta_angle = settings.control.mouse_sensitivity * delta_time;

        // Pitch and Yaw
        mouse.read().for_each(|motion| {
            if player.free_look {
                player_transform.rotate_local_x(delta_angle * -motion.delta.y);
                player_transform.rotate_local_z(delta_angle * -motion.delta.x);
            } else {
                camera.rotate_local_x(delta_angle * -motion.delta.y);
                player_transform.rotate_z(delta_angle * -motion.delta.x);
            };
        });

        // Roll (only when free looking)
        if player.free_look {
            let axis_back = Dir3::new(camera.back().xyz()).unwrap();

            if keyboard.pressed(settings.keybind.roll_counterclockwise) {
                camera.rotate_axis(axis_back, delta_time);
            }
            if keyboard.pressed(settings.keybind.roll_clockwise) {
                camera.rotate_axis(axis_back, -delta_time);
            }
        }
    }
}
