use bevy::{
    core_pipeline::{
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
        fxaa::Fxaa,
        smaa::SmaaSettings,
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
};

use crate::{settings::AntiAliasing, Settings};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TemporalAntiAliasPlugin);

        app.add_systems(
            First,
            (set_hdr, set_aa, set_ssao)
                .run_if(|settings: Res<Settings>| resource_changed(settings)),
        );

        app.insert_resource(Settings::load("settings.json").unwrap_or_else(|_| {
            let settings = Settings::default();
            settings.save("settings.json").unwrap();
            settings
        }));
    }
}

fn set_hdr(settings: Res<Settings>, mut query: Query<&mut Camera>) {
    query.iter_mut().for_each(|mut camera| {
        camera.hdr = settings.graphic.hdr;
    });
}

fn set_aa(
    mut commands: Commands,
    settings: Res<Settings>,
    mut msaa: ResMut<Msaa>,
    mut query: Query<Entity, With<Camera>>,
) {
    query.iter_mut().for_each(|entity| {
        let mut camera = commands.entity(entity);

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
    });
}

fn set_ssao(mut commands: Commands, settings: Res<Settings>, query: Query<Entity, With<Camera>>) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .remove::<ScreenSpaceAmbientOcclusionBundle>()
            .insert(ScreenSpaceAmbientOcclusionBundle {
                settings: ScreenSpaceAmbientOcclusionSettings {
                    quality_level: settings.graphic.ssao,
                },
                ..default()
            });
    });
}
