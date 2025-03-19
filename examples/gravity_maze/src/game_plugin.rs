use std::f32::consts::PI;

use bevy::prelude::*;
use rustopia_maze::MazePlugin;
use rustopia_skylight::{SkylightPlugin, SkylightSetting};

use crate::{GravityPlugin, PlayerPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkylightPlugin)
            .add_plugins(MazePlugin)
            .add_plugins(GravityPlugin)
            .add_plugins(PlayerPlugin);

        app.add_systems(Startup, setup_bgm);

        app.insert_resource(SkylightSetting {
            illuminance: 10000.,
            hour_angle: PI,
            ..default()
        });
    }
}

fn setup_bgm(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/Windless Slopes.ogg")),
        PlaybackSettings::LOOP,
    ));
}
