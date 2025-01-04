#![windows_subsystem = "windows"]

use std::fs::OpenOptions;

use bevy::{
    log::{tracing_subscriber, BoxedLayer, Level, LogPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    custom_layer,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Rustopia"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(rustopia::GamePlugin)
        .run();
}

fn custom_layer(_: &mut App) -> Option<BoxedLayer> {
    let debug_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("debug.log")
        .expect("Failed to open debug.log");

    Some(Box::new(
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(debug_file),
    ))
}
