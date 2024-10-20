use bevy::prelude::*;

use crate::DebugPanelPages;

#[derive(Default, Resource)]
pub struct DebugPanelState {
    pub enabled: bool,
    pub page: DebugPanelPages,
}
