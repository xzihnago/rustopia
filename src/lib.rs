mod debug_texture;
mod scenes;

use bevy::prelude::*;
use debug_texture::DebugTexture;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(settings::SettingsPlugin)
            .add_plugins(scenes::DebugWorldPlugin);
        // .add_plugins(scenes::PhysicsWorldPlugin);
        // .add_plugins(scenes::TerrainWorldPlugin);
    }
}
