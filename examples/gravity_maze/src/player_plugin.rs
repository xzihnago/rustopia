use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rustopia_utils::DebugTexture;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, fixed_camera_location);
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
    commands.spawn((Camera3d::default(), Msaa::Off));
}

fn fixed_camera_location(
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

#[derive(Component)]
pub struct Player;
