mod scenes;

use bevy::{prelude::*, window::CursorGrabMode};

use rustopia_settings::SettingsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SettingsPlugin);

        // app.add_plugins(scenes::MazeWorldPlugin);
        app.add_plugins(scenes::DebugWorldPlugin);
        // app.add_plugins(scenes::PhysicsWorldPlugin);
        // app.add_plugins(scenes::TerrainWorldPlugin);

        app.add_systems(PreUpdate, update_cursor_grab_mode);
    }
}

fn update_cursor_grab_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut window: Query<&mut Window>,
) {
    if let Ok(mut window) = window.get_single_mut() {
        if keyboard.just_pressed(KeyCode::Escape) {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        } else if mouse.just_pressed(MouseButton::Left) {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
    }
}
