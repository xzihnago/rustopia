use bevy::prelude::*;

use crate::DebugPanelPages;

#[derive(Bundle)]
pub struct DebugPanelPagesBundle(pub DebugPanelPages, pub TextBundle);
