use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::droplet::Droplet;
use crate::Terrain;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map_size = (100, 100);

    let mut terrain = Terrain::random(map_size, 1.);

    // No erosion
    // let mesh = terrain.calc_mesh();
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(mesh),
    //     material: materials.add(Color::GRAY),
    //     transform: Transform::from_translation(Vec3::new(
    //         map_size.0 as f32 / -2.,
    //         map_size.1 as f32 / -2.,
    //         -20.,
    //     )),
    //     ..default()
    // });

    // 100000 iterations of hydrolic erosion
    let mut rng = rand::thread_rng();
    for _ in 0..100000 {
        terrain.hydrolic_erosion(Droplet::from_rand(&mut rng, map_size));
    }

    let mesh = terrain.calc_mesh();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::LIME_GREEN),
            transform: Transform::from_translation(Vec3::new(
                map_size.0 as f32 / -2.,
                map_size.1 as f32 / -2.,
                -20.,
            )),
            ..default()
        },
        collider,
    ));
}
