mod debug_texture;
mod scenes;

use bevy::{prelude::*, window::CursorGrabMode};
use debug_texture::DebugTexture;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rustopia_settings::SettingsPlugin)
            .add_systems(PreUpdate, update_cursor_grab_mode);

        app.add_plugins(scenes::DebugWorldPlugin);
        // .add_plugins(scenes::PhysicsWorldPlugin);
        // .add_plugins(scenes::TerrainWorldPlugin);
    }
}

fn update_cursor_grab_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut window: Query<&mut Window>,
) {
    let mut window = window.single_mut();

    if keyboard.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    } else if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}
