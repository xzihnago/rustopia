use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct KeybindSettings {
    pub move_sprint: KeyCode,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_up_axis: KeyCode,
    pub move_down_axis: KeyCode,
    pub roll_counterclockwise: KeyCode,
    pub roll_clockwise: KeyCode,
}

impl Default for KeybindSettings {
    fn default() -> Self {
        Self {
            move_sprint: KeyCode::ShiftLeft,
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_up: KeyCode::Space,
            move_down: KeyCode::ControlLeft,
            move_up_axis: KeyCode::KeyX,
            move_down_axis: KeyCode::KeyZ,
            roll_counterclockwise: KeyCode::KeyQ,
            roll_clockwise: KeyCode::KeyE,
        }
    }
}
