use bevy::prelude::*;
use rand::prelude::*;

pub struct Droplet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub water: f32,
    pub sediment: f32,
}

impl Droplet {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            water: 1.,
            sediment: 0.,
        }
    }

    pub fn from_rand(rng: &mut ThreadRng, size: (usize, usize)) -> Self {
        Self::new(Vec2::new(
            rng.gen_range(1.0..=size.0 as f32 - 2.),
            rng.gen_range(1.0..=size.1 as f32 - 2.),
        ))
    }
}
