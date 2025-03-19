use bevy::{math::primitives, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct TerrainWorldPlugin;

impl Plugin for TerrainWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(terrain::TerrainPlugin)
            .insert_resource(RapierConfiguration {
                gravity: Vec3::NEG_Z * 9.81,
                ..default()
            })
            .add_systems(Startup, startup);
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Origin cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Cuboid::new(1., 1., 1.))),
        material: materials.add(StandardMaterial::from(Color::WHITE)),
        ..default()
    });

    // X cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Cuboid::new(0.5, 0.5, 0.5))),
        material: materials.add(StandardMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::X),
        ..default()
    });

    // Y cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Cuboid::new(0.5, 0.5, 0.5))),
        material: materials.add(StandardMaterial::from(Color::GREEN)),
        transform: Transform::from_translation(Vec3::Y),
        ..default()
    });

    // Z cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Cuboid::new(0.5, 0.5, 0.5))),
        material: materials.add(StandardMaterial::from(Color::BLUE)),
        transform: Transform::from_translation(Vec3::Z),
        ..default()
    });

    // Sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(primitives::Sphere::new(1.)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                specular_transmission: 0.9,
                diffuse_transmission: 1.0,
                thickness: 1.,
                perceptual_roughness: 0.12,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(1.),
        ColliderMassProperties::Mass(100.),
    ));
}
