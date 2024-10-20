use bevy::prelude::*;

#[derive(Default, Component)]
pub enum DebugPanelPages {
    #[default]
    SystemInfo,
    Camera,
    Physics,
    SkyLight,
}
