use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rustopia_settings::Settings;
use rustopia_skylight::SkylightSetting;
use rustopia_utils::DebugTexture;

pub struct MazeWorldPlugin;

impl Plugin for MazeWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rustopia_skylight::SkylightPlugin)
            // .add_plugins(rustopia_player::FreeCameraPlugin)
            .add_plugins(rustopia_maze::MazePlugin)
            .add_plugins(PlayerPlugin)
            .insert_resource(SkylightSetting {
                illuminance: 10000.,
                hour_angle: PI,
                ..default()
            });
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, (lock_camera_on_sphere, move_keyboard));
    }
}

fn startup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sphere
    commands.spawn((
        Player,
        Transform::from_translation(0.5 * Vec3::ONE),
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(materials.add(images.add(DebugTexture::colorful()))),
        RigidBody::Dynamic,
        Collider::ball(0.4),
        Sleeping {
            normalized_linear_threshold: 0.,
            angular_threshold: 0.,
            ..default()
        },
    ));

    // Camera
    commands.spawn(Camera3d::default());
}

fn lock_camera_on_sphere(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let (Ok(mut camera), Ok(player)) = (camera.get_single_mut(), player.get_single()) {
        *camera = Transform::from_xyz(
            player.translation.x,
            player.translation.y,
            player.translation.z + 10.,
        );
    }
}

fn move_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut physic: Query<&mut RapierConfiguration>,
) {
    let mut physic = physic.single_mut();

    let mut direction = 4. * Vec3::NEG_Z;

    if keyboard.pressed(settings.keybind.move_forward) {
        direction += Vec3::Y;
    }
    if keyboard.pressed(settings.keybind.move_backward) {
        direction += Vec3::NEG_Y;
    }
    if keyboard.pressed(settings.keybind.move_right) {
        direction += Vec3::X;
    }
    if keyboard.pressed(settings.keybind.move_left) {
        direction += Vec3::NEG_X;
    }

    physic.gravity = 9.81 * direction.normalize();
}

#[derive(Component)]
pub struct Player;
