use bevy::{
    core_pipeline::{
        experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasing},
        fxaa::Fxaa,
        smaa::Smaa,
    },
    pbr::ScreenSpaceAmbientOcclusion,
    prelude::*,
    window::PresentMode,
};

use crate::{
    settings::{AntiAliasing, VsyncMode},
    Settings,
};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TemporalAntiAliasPlugin);

        app.add_systems(
            First,
            (
                save_settings,
                set_window_mode,
                set_resolution,
                set_vsync,
                set_hdr,
                set_aa,
                set_ssao,
            )
                .run_if(|settings: Res<Settings>| resource_changed(settings)),
        );

        app.insert_resource(Settings::load("settings.json").unwrap_or_default());
    }
}

fn save_settings(settings: Res<Settings>) {
    settings
        .save("settings.json")
        .expect("Failed to save settings");
}

fn set_window_mode(mut window: Query<&mut Window>, settings: Res<Settings>) {
    window.iter_mut().for_each(|mut window| {
        window.mode = settings.graphic.window_mode;
    });
}

fn set_resolution(mut window: Query<&mut Window>, settings: Res<Settings>) {
    window.iter_mut().for_each(|mut window| {
        window.resolution.set_physical_resolution(
            settings.graphic.resolution.width,
            settings.graphic.resolution.height,
        );
        window.position.center(MonitorSelection::Current);
    });
}

fn set_vsync(mut window: Query<&mut Window>, settings: Res<Settings>) {
    window.iter_mut().for_each(|mut window| {
        window.present_mode = match settings.graphic.vsync {
            VsyncMode::Off => PresentMode::AutoNoVsync,
            VsyncMode::On => PresentMode::Fifo,
            VsyncMode::Auto => PresentMode::AutoVsync,
            VsyncMode::Adaptive => PresentMode::FifoRelaxed,
            VsyncMode::Fast => PresentMode::Mailbox,
        }
    });
}

fn set_hdr(settings: Res<Settings>, mut query: Query<&mut Camera>) {
    query.iter_mut().for_each(|mut camera| {
        camera.hdr = settings.graphic.hdr;
    });
}

fn set_aa(mut commands: Commands, settings: Res<Settings>, mut query: Query<Entity, With<Camera>>) {
    query.iter_mut().for_each(|entity| {
        let mut camera = commands.entity(entity);

        camera.remove::<(Fxaa, Msaa, TemporalAntiAliasing)>();

        match settings.graphic.aa {
            AntiAliasing::FXAA(sensitivity) => {
                camera.insert(Fxaa {
                    edge_threshold: sensitivity,
                    edge_threshold_min: sensitivity,
                    ..default()
                });
            }

            AntiAliasing::SMAA(preset) => {
                camera.insert(Smaa { preset });
            }

            AntiAliasing::MSAA(samples) => {
                camera.insert(samples);
            }

            AntiAliasing::TAA => {
                camera.insert(TemporalAntiAliasing::default());
            }

            _ => {}
        }
    });
}

fn set_ssao(mut commands: Commands, settings: Res<Settings>, query: Query<Entity, With<Camera>>) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .remove::<ScreenSpaceAmbientOcclusion>()
            .insert(ScreenSpaceAmbientOcclusion {
                quality_level: settings.graphic.ssao,
                ..default()
            });
    });
}
