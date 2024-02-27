use bevy::prelude::*;

use crate::Settings;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings::default());
    }
}
