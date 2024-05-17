use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use crate::SkylightSetting;
use crate::SkylightTimer;

pub struct SkylightPlugin;

impl Plugin for SkylightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(Startup, startup)
            .add_systems(PostStartup, post_startup)
            .add_systems(
                PreUpdate,
                |time: Res<Time>, mut timer: ResMut<SkylightTimer>| timer.tick(time.delta()),
            )
            .add_systems(
                Update,
                update.run_if(|timer: Res<SkylightTimer>| timer.finished()),
            );
    }
}

fn startup(mut commands: Commands) {
    commands.insert_resource(SkylightTimer::default());

    let setting = SkylightSetting::default();
    let data = setting.calc_skylight_data();

    commands.insert_resource(AtmosphereModel::new(Nishita {
        ray_origin: Vec3::new(0., 0., 6372e3),
        sun_position: data.solar,
        ..default()
    }));

    commands.insert_resource(AmbientLight {
        color: Color::ALICE_BLUE,
        brightness: data.brightness,
    });

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: data.illuminance,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::IDENTITY.looking_to(-data.solar, data.axis),
            ..default()
        },
        setting,
    ));
}

fn post_startup(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    let mut camera = commands.entity(camera.single());
    camera.insert(AtmosphereCamera::default());
}

fn update(
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    mut atmosphere: AtmosphereMut<Nishita>,
    mut skylight: Query<(&mut Transform, &mut DirectionalLight, &mut SkylightSetting)>,
    camera: Query<&GlobalTransform, With<Camera>>,
) {
    let (mut transform, mut light, mut setting) = skylight.single_mut();
    let camera = camera.single();

    // Update the hour angle
    setting.hour_angle = setting.angvel * time.elapsed_seconds_wrapped() - 2.;
    let data = setting.calc_skylight_data();

    // Update the atmosphere and light
    ambient.brightness = data.brightness;

    atmosphere.ray_origin = Vec3::new(0., 0., 6372e3 + camera.translation().z);

    atmosphere.sun_position = data.solar;
    transform.look_to(-data.solar, data.axis);
    light.illuminance = data.illuminance;
    light.color = data.color;
}
