use std::f32::consts::{FRAC_2_PI, TAU};

use bevy::prelude::*;

use crate::utils::*;

#[derive(Component)]
pub struct SkylightSetting {
    pub illuminance: f32,
    pub angvel: f32,

    pub inclination: f32,
    pub latitude: f32,
    pub hour_angle: f32,

    pub colortemp_min: f32,
    pub colortemp_max: f32,

    north: Vec3,
    east: Vec3,
    down: Vec3,
}

impl SkylightSetting {
    pub fn calc_skylight_data(&self) -> SkylightData {
        let axis = Quat::from_axis_angle(self.east, self.latitude) * self.north;

        let solar = Quat::from_axis_angle(axis, self.hour_angle)
            * Quat::from_axis_angle(self.east, self.latitude + self.inclination)
            * self.down;

        let brightness = map_range((-0.15, 0.), (10., 80.), solar.z.clamp(-0.15, 0.));

        let illuminance = self.illuminance * solar.z.max(0.);

        let curve = (-(solar.z.acos() * FRAC_2_PI).powf(4.) + 1.).max(0.); // y = -x^4 + 1
        let kelvin = (self.colortemp_max - self.colortemp_min) * curve + self.colortemp_min;
        let color = color_from_temperature(kelvin);

        SkylightData {
            axis,
            solar,
            brightness,
            illuminance,
            color,
        }
    }
}

impl Default for SkylightSetting {
    fn default() -> Self {
        Self {
            illuminance: 100000.,
            angvel: -TAU * 30. / 86400.,

            inclination: 23.45_f32.to_radians(),
            latitude: 23.45_f32.to_radians(),
            hour_angle: 0.,

            colortemp_min: 1850.,
            colortemp_max: 6500.,

            north: Vec3::Y,
            east: Vec3::X,
            down: Vec3::NEG_Z,
        }
    }
}

#[derive(Debug)]
pub struct SkylightData {
    pub axis: Vec3,
    pub solar: Vec3,
    pub brightness: f32,
    pub illuminance: f32,
    pub color: Color,
}
