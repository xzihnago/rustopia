use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rustopia_skylight::SkylightSetting;
use rustopia_utils::{AutoCollider, DebugTexture};

pub struct DebugWorldPlugin;

impl Plugin for DebugWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rustopia_skylight::SkylightPlugin)
            .add_plugins(rustopia_player::FreeCameraPlugin)
            .add_systems(Startup, startup)
            .add_systems(PreUpdate, update_skylight_setting);
    }
}

fn startup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut physic: Query<&mut RapierConfiguration>,
) {
    // Physics configuration
    let mut physic = physic.single_mut();
    physic.gravity = 9.81 * Vec3::NEG_Z;

    // Ground
    let mesh = Plane3d::new(Vec3::Z, 50. * Vec2::ONE);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(images.add(DebugTexture::checkerboard(100, 100)))),
        AutoCollider,
    ));

    // Origin cube
    let mesh = Cuboid::new(1., 1., 1.);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::WHITE)),
        AutoCollider,
    ));

    // XYZ cube
    let mesh = Cuboid::new(0.5, 0.5, 0.5);
    commands.spawn((
        Transform::from_translation(Vec3::X),
        Mesh3d(meshes.add(mesh.clone())),
        MeshMaterial3d(materials.add(Color::from(bevy::color::palettes::css::RED))),
        AutoCollider,
    ));
    commands.spawn((
        Transform::from_translation(Vec3::Y),
        Mesh3d(meshes.add(mesh.clone())),
        MeshMaterial3d(materials.add(Color::from(bevy::color::palettes::css::GREEN))),
        AutoCollider,
    ));
    commands.spawn((
        Transform::from_translation(Vec3::Z),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::from(bevy::color::palettes::css::BLUE))),
        AutoCollider,
    ));

    // Sphere
    commands.spawn((
        Transform::from_translation(Vec3::new(10., 0., 10.)),
        Mesh3d(meshes.add(Sphere::new(1.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            perceptual_roughness: 0.,
            reflectance: 1.,
            diffuse_transmission: 1.,
            specular_transmission: 1.,
            thickness: 0.5,
            ..default()
        })),
        RigidBody::Dynamic,
        Collider::ball(1.),
        ColliderMassProperties::Mass(1000.),
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(14., 0., 10.)),
        Mesh3d(meshes.add(Sphere::new(1.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            perceptual_roughness: 0.,
            reflectance: 1.,
            diffuse_transmission: 1.,
            specular_transmission: 1.,
            thickness: 0.5,
            ..default()
        })),
        RigidBody::Dynamic,
        Collider::ball(1.),
        ColliderMassProperties::Mass(1000.),
    ));
}

fn update_skylight_setting(time: Res<Time>, mut setting: ResMut<SkylightSetting>) {
    setting.inclination = 23.45_f32.to_radians();
    setting.latitude = 23.45_f32.to_radians();
    setting.hour_angle = -TAU * 30. / 86400. * time.elapsed_secs_wrapped() - 2.;
}
