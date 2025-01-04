mod scenes;

use bevy::{prelude::*, window::CursorGrabMode};

use rustopia_debugpanel::DebugPanelPlugin;
use rustopia_settings::SettingsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SettingsPlugin)
            .add_plugins(DebugPanelPlugin);

        app.add_plugins(scenes::MazeWorldPlugin);
        // app.add_plugins(scenes::DebugWorldPlugin);
        // app.add_plugins(scenes::PhysicsWorldPlugin);
        // app.add_plugins(scenes::TerrainWorldPlugin);

        app.add_systems(PostStartup, insert_default_ui_camera)
            .add_systems(PreUpdate, update_cursor_grab_mode);
    }
}

fn insert_default_ui_camera(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    commands.entity(query.single()).insert(IsDefaultUiCamera);
}

fn update_cursor_grab_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut window: Query<&mut Window>,
) {
    if let Ok(mut window) = window.get_single_mut() {
        if keyboard.just_pressed(KeyCode::Escape) {
            window.cursor_options.visible = true;
            window.cursor_options.grab_mode = CursorGrabMode::None;
        } else if mouse.just_pressed(MouseButton::Left) {
            window.cursor_options.visible = false;
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
        }
    }
}
