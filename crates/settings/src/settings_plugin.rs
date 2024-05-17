use bevy::{
    core_pipeline::{
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
        fxaa::Fxaa,
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
};

use crate::{AntiAliasing, Settings};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = Settings::load("settings.json").unwrap_or_default();
        settings.save("settings.json").unwrap();

        app.add_plugins(TemporalAntiAliasPlugin)
            .insert_resource(settings)
            .add_systems(PostStartup, (set_aa, set_ssao));
    }
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
