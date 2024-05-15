use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ControlSettings {
    pub move_speed: f32,
    pub rotate_speed: f32,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            move_speed: 400.,
            rotate_speed: 0.2,
        }
    }
}
