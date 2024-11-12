use bevy::prelude::*;
use maze_generator::{prelude::Direction, recursive_backtracking::RbGenerator};

use crate::{Maze, MazeSetting};
use rustopia_utils::AutoCollider;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, generate_maze)
            .add_systems(Startup, spawn_maze);

        app.init_resource::<MazeSetting>();
    }
}

fn generate_maze(mut setting: ResMut<MazeSetting>) {
    let generator = RbGenerator::new(setting.seed);
    setting.generate(generator);
}

fn spawn_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    setting: Res<MazeSetting>,
) {
    commands
        .spawn((Maze, SpatialBundle::default()))
        .with_children(|parent| {
            let width = setting.size * setting.width as f32;
            let height = setting.size * setting.height as f32;

            // Ground
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(width, height, setting.thickness)),
                    material: materials.add(setting.color_ground),
                    transform: Transform::from_xyz(width * 0.5, height * 0.5, 0.),
                    ..default()
                },
                AutoCollider,
            ));
        })
        .with_children(|parent| {
            let mesh = Cuboid::new(setting.size, setting.size, setting.thickness);

            // Start
            parent.spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(setting.color_start),
                transform: Transform::from_xyz(setting.size * 0.5, setting.size * 0.5, 0.0001),
                ..default()
            });

            // Goal
            parent.spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(setting.color_goal),
                transform: Transform::from_xyz(
                    setting.size * (setting.width as f32 - 0.5),
                    setting.size * (setting.height as f32 - 0.5),
                    0.0001,
                ),
                ..default()
            });
        })
        .with_children(|parent| {
            // X walls
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(
                        setting.size * setting.width as f32 + setting.thickness,
                        setting.size + setting.thickness,
                        setting.thickness,
                    )),
                    material: materials.add(setting.color_wall),
                    transform: Transform::from_translation(
                        setting.size * Vec3::new(0.5 * setting.width as f32, 0., 0.5),
                    )
                    .looking_to(Vec3::Y, Vec3::Z),
                    ..default()
                },
                AutoCollider,
            ));

            for y in 0..setting.height {
                let mut count = 0;

                for mut x in 0..setting.width {
                    let field = setting.get(x, y).unwrap();

                    if !field.has_passage(&Direction::South) {
                        count += 1;
                        if x == setting.width - 1 {
                            x += 1;
                        }
                    }

                    if (field.has_passage(&Direction::South) && count > 0) || (x == setting.width) {
                        parent.spawn((
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(
                                    setting.size * count as f32 + setting.thickness,
                                    setting.size + setting.thickness,
                                    setting.thickness,
                                )),
                                material: materials.add(setting.color_wall),
                                transform: Transform::from_translation(
                                    setting.size
                                        * Vec3::new(
                                            x as f32 - (count as f32 * 0.5),
                                            (y + 1) as f32,
                                            0.5,
                                        ),
                                )
                                .looking_to(Vec3::Y, Vec3::Z),
                                ..default()
                            },
                            AutoCollider,
                        ));

                        count = 0;
                    }
                }
            }
        })
        .with_children(|parent| {
            // Y walls
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(
                        setting.size * setting.width as f32 + setting.thickness,
                        setting.size + setting.thickness,
                        setting.thickness,
                    )),
                    material: materials.add(setting.color_wall),
                    transform: Transform::from_translation(
                        setting.size * Vec3::new(0., 0.5 * setting.height as f32, 0.5),
                    )
                    .looking_to(Vec3::X, Vec3::Z),
                    ..default()
                },
                AutoCollider,
            ));

            for x in 0..setting.width {
                let mut count = 0;

                for mut y in 0..setting.height {
                    let field = setting.get(x, y).unwrap();

                    if !field.has_passage(&Direction::East) {
                        count += 1;
                        if y == setting.height - 1 {
                            y += 1;
                        }
                    }

                    if (field.has_passage(&Direction::East) && count > 0) || (y == setting.height) {
                        parent.spawn((
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(
                                    setting.size * count as f32 + setting.thickness,
                                    setting.size + setting.thickness,
                                    setting.thickness,
                                )),
                                material: materials.add(setting.color_wall),
                                transform: Transform::from_translation(
                                    setting.size
                                        * Vec3::new(
                                            (x + 1) as f32,
                                            y as f32 - (count as f32 * 0.5),
                                            0.5,
                                        ),
                                )
                                .looking_to(Vec3::X, Vec3::Z),
                                ..default()
                            },
                            AutoCollider,
                        ));

                        count = 0;
                    }
                }
            }
        });
}
