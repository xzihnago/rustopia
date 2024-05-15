use std::fs::File;

use anyhow::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{ControlSettings, GraphicsSettings, KeybindSettings};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub graphic: GraphicsSettings,
    pub control: ControlSettings,
    pub keybind: KeybindSettings,
}

impl Settings {
    pub fn load(path: &str) -> Result<Self> {
        Ok(serde_json::from_reader(File::open(path)?).unwrap_or_default())
    }

    pub fn save(&self, path: &str) -> Result<()> {
        Ok(serde_json::to_writer(File::create(path)?, self)?)
    }
}
