use bevy::{
    core_pipeline::{
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
        fxaa::Fxaa,
        smaa::SmaaSettings,
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
};

use crate::{AntiAliasing, Settings};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TemporalAntiAliasPlugin)
            .add_systems(PostStartup, (set_hdr, set_aa, set_ssao))
            .insert_resource(Settings::load("settings.json").unwrap_or_else(|_| {
                let settings = Settings::default();
                settings.save("settings.json").unwrap();
                settings
            }));
    }
}

fn set_hdr(settings: Res<Settings>, mut camera: Query<&mut Camera>) {
    let mut camera = camera.single_mut();
    camera.hdr = settings.graphic.hdr;
}

fn set_aa(
    mut commands: Commands,
    settings: Res<Settings>,
    mut msaa: ResMut<Msaa>,
    mut camera: Query<Entity, With<Camera>>,
) {
    let mut camera = commands.entity(camera.single_mut());

    *msaa = Msaa::Off;
    camera.remove::<(Fxaa, TemporalAntiAliasBundle)>();

    match settings.graphic.aa {
        AntiAliasing::FXAA(sensitivity) => {
            camera.insert(Fxaa {
                edge_threshold: sensitivity,
                edge_threshold_min: sensitivity,
                ..default()
            });
        }

        AntiAliasing::SMAA(preset) => {
            camera.insert(SmaaSettings { preset });
        }

        AntiAliasing::MSAA(samples) => {
            *msaa = samples;
        }

        AntiAliasing::TAA => {
            camera.insert(TemporalAntiAliasBundle::default());
        }

        _ => {}
    }
}

fn set_ssao(mut commands: Commands, settings: Res<Settings>, camera: Query<Entity, With<Camera>>) {
    let mut camera = commands.entity(camera.single());

    camera.remove::<ScreenSpaceAmbientOcclusionBundle>();
    camera.insert(ScreenSpaceAmbientOcclusionBundle {
        settings: ScreenSpaceAmbientOcclusionSettings {
            quality_level: settings.graphic.ssao,
        },
        ..default()
    });
}
