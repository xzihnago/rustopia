use bevy::{
    core_pipeline::{
        experimental::taa::{
            TemporalAntiAliasBundle, TemporalAntiAliasPlugin, TemporalAntiAliasSettings,
        },
        fxaa::Fxaa,
    },
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
            .add_systems(PostStartup, set_anti_aliasing);
    }
}

fn set_anti_aliasing(
    mut commands: Commands,
    settings: Res<Settings>,
    mut msaa: ResMut<Msaa>,
    mut camera: Query<
        (
            Entity,
            Option<&mut Fxaa>,
            Option<&TemporalAntiAliasSettings>,
        ),
        With<Camera>,
    >,
) {
    let (camera_entity, fxaa, taa) = camera.single_mut();
    let mut camera = commands.entity(camera_entity);

    match settings.graphic.anti_aliasing {
        AntiAliasing::Off => {
            *msaa = Msaa::Off;
            camera.remove::<Fxaa>();
            camera.remove::<TemporalAntiAliasBundle>();
        }

        AntiAliasing::FXAA(sensitivity)
            if fxaa.is_none() || fxaa.unwrap().edge_threshold != sensitivity =>
        {
            *msaa = Msaa::Off;
            camera.remove::<Fxaa>();
            camera.remove::<TemporalAntiAliasBundle>();
            camera.insert(Fxaa {
                edge_threshold: sensitivity,
                edge_threshold_min: sensitivity,
                ..default()
            });
        }

        AntiAliasing::MSAA(samples) if *msaa != samples => {
            *msaa = samples;
            camera.remove::<Fxaa>();
            camera.remove::<TemporalAntiAliasBundle>();
        }

        AntiAliasing::TAA if taa.is_none() => {
            *msaa = Msaa::Off;
            camera.remove::<Fxaa>();
            camera.insert(TemporalAntiAliasBundle::default());
        }

        _ => {}
    }
}
