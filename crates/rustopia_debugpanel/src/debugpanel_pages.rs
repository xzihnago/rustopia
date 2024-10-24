use bevy::prelude::*;
use strum_macros::{AsRefStr, EnumIter};

#[derive(Clone, Copy, PartialEq, Default, EnumIter, AsRefStr, Component)]
pub enum DebugPanelPages {
    #[default]
    System,
    Info,
    Camera,
    Physics,
    SkyLight,
}
