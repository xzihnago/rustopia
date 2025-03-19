use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

use crate::DebugTexture;

pub struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            // gravity: Vec3::NEG_Z * 9.81,
            gravity: Vec3::ZERO,
            ..default()
        })
        .add_systems(FixedUpdate, (update_drag, update_gravity));

        // Spawn objects
        app.add_systems(Startup, spawn_room);
        for _ in 0..1000 {
            app.add_systems(Startup, spawn_sphere);
        }
    }
}

#[derive(Component)]
struct Ball;

fn update_drag(mut query: Query<(&Collider, &Velocity, &mut Damping), With<Ball>>) {
    query
        .par_iter_mut()
        .for_each(|(collider, velocity, mut damping)| {
            let radius = collider.as_ball().unwrap().radius();
            let area = PI * radius * radius;

            // F = 1/2 * p * Cd * A * v^2,  p: 1.225 kg/m^3 @ 15°C,  Cd: 0.47,  limit force to 1‰
            let drag = 0.001 * 0.5 * 1.225 * 0.47 * area * velocity.linvel.length_squared();

            damping.linear_damping = drag;
        });
}

fn update_gravity(
    mut query_this: Query<(&Transform, &ReadMassProperties, &mut ExternalForce), With<Ball>>,
    query_other: Query<(&Transform, &ReadMassProperties), With<Ball>>,
) {
    query_this
        .par_iter_mut()
        .for_each(|(transform_this, mass_this, mut force)| {
            force.force = Vec3::ZERO;

            for (transform_other, mass_other) in &query_other {
                if std::ptr::eq(transform_this, transform_other) {
                    continue;
                }

                let distance = transform_other.translation - transform_this.translation;

                // F = G * m1 * m2 / r^2, ignore G
                let gravity = mass_this.mass * mass_other.mass / distance.length_squared();

                force.force += distance.normalize() * gravity;
            }
        });
}

fn spawn_room(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    });

    let hx = 500.;
    let hy = 500.;
    let restitution = 1.;

    // Mesh
    let mesh = meshes.add(Plane3d::new(Vec3::Z).mesh().size(hx * 2., hy * 2.));

    // Material
    // let material = materials.add(Color::LIME_GREEN.into());
    let material = materials.add(images.add(DebugTexture::colorful()));

    // Spawn
    let mut transforms = [
        Transform::from_xyz(0., 0., -500.), // Floor
        Transform::from_xyz(0., 0., 500.),  // Ceiling
        Transform::from_xyz(-hx, 0., 0.),   // Wall (-x)
        Transform::from_xyz(hx, 0., 0.),    // Wall (+x)
        Transform::from_xyz(0., -hy, 0.),   // Wall (-y)
        Transform::from_xyz(0., hy, 0.),    // Wall (+y)
    ];

    transforms[1].rotate(Quat::from_euler(EulerRot::XYZ, PI, 0., 0.));
    transforms[2].rotate(Quat::from_euler(EulerRot::XYZ, 0., PI / 2., 0.));
    transforms[3].rotate(Quat::from_euler(EulerRot::XYZ, 0., -PI / 2., 0.));
    transforms[4].rotate(Quat::from_euler(EulerRot::XYZ, -PI / 2., 0., 0.));
    transforms[5].rotate(Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.));

    for transform in transforms {
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform,
                ..default()
            },
            Collider::trimesh(
                vec![
                    [-hx, -hy, 0.].into(),
                    [hx, -hy, 0.].into(),
                    [hx, hy, 0.].into(),
                    [-hx, hy, 0.].into(),
                ],
                vec![[0, 1, 3], [1, 2, 3]],
            ),
            Restitution::coefficient(restitution),
        ));
    }
}

fn spawn_sphere(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let radius = 1.;
    let restitution = 0.9;
    let density = 5.5134;

    let angle = rng.gen_range(0.0..PI * 2.);
    let transform = Transform::from_xyz(
        angle.cos() * rng.gen_range(50.0..400.0),
        angle.sin() * rng.gen_range(50.0..400.0),
        rng.gen_range(-10.0..10.0),
    );
    let velocity =
        Vec3::Z.cross(transform.translation) / transform.translation.length_squared() * 1000.;

    // Mesh
    let mesh = meshes.add(Sphere::new(radius).mesh().ico(5).unwrap());

    // Material
    let material = materials.add(images.add(DebugTexture::colorful()));

    // Spawn
    commands.spawn((
        Ball,
        PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        },
        // Rigid body
        RigidBody::Dynamic,
        Velocity::linear(velocity),
        ExternalForce::default(),
        Damping::default(),
        // Collider
        Collider::ball(radius),
        ColliderMassProperties::Density(density),
        Restitution::coefficient(restitution),
        ReadMassProperties::default(),
    ));
}
