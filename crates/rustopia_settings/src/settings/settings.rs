use std::fs::File;

use anyhow::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{ControlSettings, GraphicSettings, KeybindSettings};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub graphic: GraphicSettings,
    pub control: ControlSettings,
    pub keybind: KeybindSettings,
}

impl Settings {
    pub fn load(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        Ok(serde_json::to_writer(file, self)?)
    }
}
