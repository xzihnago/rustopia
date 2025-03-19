use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub is_grounded: bool,
    pub free_look: bool,
    pub free_move: bool,
    pub move_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_grounded: false,
            free_look: false,
            free_move: false,
            move_speed: 400.,
        }
    }
}

#[derive(Component)]
pub struct PlayerGroundSensor;
