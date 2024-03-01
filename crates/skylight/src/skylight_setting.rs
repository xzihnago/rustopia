use std::f32::consts::TAU;

use bevy::prelude::*;

#[derive(Component)]
pub struct SkylightSetting {
    pub illuminance: f32,
    pub angvel: f32,

    pub inclination: f32,
    pub latitude: f32,
    pub hour_angle: f32,

    pub colortemp_base: f32,
    pub colortemp_interval: f32,
}

impl Default for SkylightSetting {
    fn default() -> Self {
        Self {
            illuminance: 100000.,
            angvel: -TAU * 30. / 86400.,

            inclination: 23.45_f32.to_radians(),
            latitude: 23.45_f32.to_radians(),
            hour_angle: 0.,

            colortemp_base: 1850.,
            colortemp_interval: 4650., // 6500 - 1850
        }
    }
}
