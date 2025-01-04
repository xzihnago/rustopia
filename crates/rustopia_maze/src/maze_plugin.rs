use bevy::prelude::*;
use maze_generator::prelude::*;

use crate::Maze;
use rustopia_utils::AutoCollider;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_maze, spawn_maze.after(generate_maze)));
    }
}

pub fn generate_maze(mut commands: Commands) {
    commands.spawn(Maze::default());
}

pub fn spawn_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Maze)>,
) {
    let (entity, maze) = query.single();

    commands
        .entity(entity)
        .with_children(|parent| {
            let width = maze.size * maze.width as f32;
            let height = maze.size * maze.height as f32;

            // Ground
            parent.spawn((
                Transform::from_xyz(width * 0.5, height * 0.5, 0.),
                Mesh3d(meshes.add(Cuboid::new(width, height, maze.thickness))),
                MeshMaterial3d(materials.add(maze.color_ground)),
                AutoCollider,
            ));
        })
        .with_children(|parent| {
            let mesh = Cuboid::new(maze.size, maze.size, maze.thickness);

            // Start
            parent.spawn((
                Transform::from_xyz(maze.size * 0.5, maze.size * 0.5, 0.0001),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(maze.color_start)),
            ));

            // Goal
            parent.spawn((
                Transform::from_xyz(
                    maze.size * (maze.width as f32 - 0.5),
                    maze.size * (maze.height as f32 - 0.5),
                    0.0001,
                ),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(maze.color_goal)),
            ));
        })
        .with_children(|parent| {
            // X walls
            parent.spawn((
                Transform::from_translation(
                    maze.size * Vec3::new(0.5 * maze.width as f32, 0., 0.5),
                )
                .looking_to(Vec3::Y, Vec3::Z),
                Mesh3d(meshes.add(Cuboid::new(
                    maze.size * maze.width as f32 + maze.thickness,
                    maze.size + maze.thickness,
                    maze.thickness,
                ))),
                MeshMaterial3d(materials.add(maze.color_wall)),
                AutoCollider,
            ));

            for y in 0..maze.height {
                let mut count = 0;

                for mut x in 0..maze.width {
                    let field = maze.get(x, y).unwrap();

                    if !field.has_passage(&Direction::South) {
                        count += 1;
                        if x == maze.width - 1 {
                            x += 1;
                        }
                    }

                    if (field.has_passage(&Direction::South) && count > 0) || (x == maze.width) {
                        parent.spawn((
                            Transform::from_translation(
                                maze.size
                                    * Vec3::new(
                                        x as f32 - (count as f32 * 0.5),
                                        (y + 1) as f32,
                                        0.5,
                                    ),
                            )
                            .looking_to(Vec3::Y, Vec3::Z),
                            Mesh3d(meshes.add(Cuboid::new(
                                maze.size * count as f32 + maze.thickness,
                                maze.size + maze.thickness,
                                maze.thickness,
                            ))),
                            MeshMaterial3d(materials.add(maze.color_wall)),
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
                Transform::from_translation(
                    maze.size * Vec3::new(0., 0.5 * maze.height as f32, 0.5),
                )
                .looking_to(Vec3::X, Vec3::Z),
                Mesh3d(meshes.add(Cuboid::new(
                    maze.size * maze.width as f32 + maze.thickness,
                    maze.size + maze.thickness,
                    maze.thickness,
                ))),
                MeshMaterial3d(materials.add(maze.color_wall)),
                AutoCollider,
            ));

            for x in 0..maze.width {
                let mut count = 0;

                for mut y in 0..maze.height {
                    let field = maze.get(x, y).unwrap();

                    if !field.has_passage(&Direction::East) {
                        count += 1;
                        if y == maze.height - 1 {
                            y += 1;
                        }
                    }

                    if (field.has_passage(&Direction::East) && count > 0) || (y == maze.height) {
                        parent.spawn((
                            Transform::from_translation(
                                maze.size
                                    * Vec3::new(
                                        (x + 1) as f32,
                                        y as f32 - (count as f32 * 0.5),
                                        0.5,
                                    ),
                            )
                            .looking_to(Vec3::X, Vec3::Z),
                            Mesh3d(meshes.add(Cuboid::new(
                                maze.size * count as f32 + maze.thickness,
                                maze.size + maze.thickness,
                                maze.thickness,
                            ))),
                            MeshMaterial3d(materials.add(maze.color_wall)),
                            AutoCollider,
                        ));

                        count = 0;
                    }
                }
            }
        });
}
