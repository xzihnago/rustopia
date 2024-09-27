use bevy::prelude::*;

pub struct SkylightData {
    pub up: Vec3,
    pub solar: Vec3,
    pub ambient: f32,
    pub directional: f32,
    pub color: Color,
}
