use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use crate::{utils::*, SkylightLight, SkylightSetting, SkylightTimer};

pub struct SkylightPlugin;

impl Plugin for SkylightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(
                PostStartup,
                |mut commands: Commands, camera: Query<Entity, With<Camera>>| {
                    camera.iter().for_each(|entity| {
                        commands.entity(entity).insert(AtmosphereCamera::default());
                    });
                },
            )
            .add_systems(
                PreUpdate,
                |time: Res<Time>, mut timer: ResMut<SkylightTimer>| {
                    timer.tick(time.delta());
                },
            )
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                update.run_if(|timer: Res<SkylightTimer>| timer.finished()),
            );
    }
}

fn startup(mut commands: Commands) {
    commands.insert_resource(SkylightTimer::default());
    commands.insert_resource(SkylightSetting::default());
    commands.insert_resource(AtmosphereModel::default());
    commands.insert_resource(AmbientLight {
        color: color_from_temperature(12000.),
        ..default()
    });

    commands.spawn((
        SkylightLight,
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));
}

fn update(
    setting: Res<SkylightSetting>,
    mut ambient: ResMut<AmbientLight>,
    mut directional: Query<(&mut DirectionalLight, &mut Transform), With<SkylightLight>>,
    mut atmosphere: AtmosphereMut<Nishita>,
    camera: Query<&GlobalTransform, With<Camera>>,
) {
    let data = setting.compute();

    // Update light
    let (mut directional, mut directional_transform) = directional.single_mut();

    ambient.brightness = data.brightness;
    directional.illuminance = data.illuminance;
    directional.color = data.color;
    directional_transform.look_to(-data.solar, data.axis);

    // Update skybox
    atmosphere.ray_origin = Vec3::new(0., 0., 6372e3 + camera.single().translation().z);
    atmosphere.sun_position = data.solar;
}
