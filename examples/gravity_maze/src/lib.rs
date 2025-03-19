mod android_sensor;
mod game_plugin;
mod gravity_plugin;
mod player_plugin;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{AppLifecycle, WindowMode},
    winit::WinitSettings,
};
use bevy_rapier3d::prelude::*;

use android_sensor::get_sensor_event;
use game_plugin::GamePlugin;
use gravity_plugin::GravityPlugin;
use player_plugin::PlayerPlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: String::from("wgpu=error,bevy_render=info,bevy_ecs=trace"),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                // ! GLES still cause wgpu panic on bevy 0.15, manually set to Vulkan
                // ! https://github.com/bevyengine/bevy/issues/10945
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(GamePlugin)
        .add_systems(Update, handle_lifetime)
        // Make the winit loop wait more aggressively when no user input is received
        // This can help reduce cpu usage on mobile devices
        .insert_resource(WinitSettings::mobile())
        .run();
}

fn handle_lifetime(mut lifecycle: EventReader<AppLifecycle>, audio_sink: Query<&AudioSink>) {
    if let Ok(audio_sink) = audio_sink.get_single() {
        for event in lifecycle.read() {
            match event {
                AppLifecycle::Suspended => audio_sink.pause(),
                AppLifecycle::Running => audio_sink.play(),
                _ => {}
            }
        }
    };
}
