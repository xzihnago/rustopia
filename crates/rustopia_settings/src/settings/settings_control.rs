use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ControlSettings {
    pub mouse_sensitivity: f32,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.2,
        }
    }
}
