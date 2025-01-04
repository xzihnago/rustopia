use bevy::{
    color::palettes::css::{GREEN, RED},
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use maze_generator::{prelude::*, recursive_backtracking::RbGenerator};

#[derive(Component)]
#[require(Visibility, Transform)]
#[component(on_add = generate_maze_hook)]
pub struct Maze {
    pub seed: Option<[u8; 32]>,
    pub width: usize,
    pub height: usize,
    pub size: f32,
    pub thickness: f32,

    pub color_ground: Color,
    pub color_wall: Color,
    pub color_start: Color,
    pub color_goal: Color,

    pub maze: Option<maze_generator::prelude::Maze>,
}

impl Default for Maze {
    fn default() -> Self {
        Self {
            seed: None,
            width: 10,
            height: 10,
            size: 1.,
            thickness: 0.1,

            color_ground: Color::WHITE,
            color_wall: Color::WHITE,
            color_start: Color::from(GREEN),
            color_goal: Color::from(RED),

            maze: None,
        }
    }
}

impl Maze {
    pub fn generate(&mut self, mut generator: impl Generator) {
        self.maze = generator
            .generate(self.width as i32, self.height as i32)
            .ok();
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Field> {
        self.maze.as_ref()?.get_field(&(x as i32, y as i32).into())
    }
}

fn generate_maze_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let mut maze = world.get_mut::<Maze>(entity).unwrap();
    let generator = RbGenerator::new(maze.seed);
    maze.generate(generator);
}
