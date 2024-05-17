use std::{
    f32::{
        consts::{FRAC_2_PI, TAU},
        EPSILON,
    },
    ops::{Add, Div, Mul, Sub},
};

use bevy::prelude::*;
use colortemp::temp_to_rgb;

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
        let colortemp = temp_to_rgb(kelvin as i64);
        let color = Color::rgb_u8(colortemp.r as u8, colortemp.g as u8, colortemp.b as u8);

        SkylightData {
            axis,
            solar,
            brightness,
            illuminance,
            color,
        }
    }
}

impl SkylightSetting {
    #[allow(dead_code)]
    pub fn new(north: Vec3, east: Vec3) -> Self {
        if north == Vec3::ZERO || east == Vec3::ZERO {
            panic!("North and east vectors must not be zero");
        } else if north.dot(east).abs() > EPSILON {
            panic!("North and east vectors must be orthogonal");
        }

        Self {
            north,
            east,
            down: north.cross(east),
            ..default()
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

fn map_range<T: Copy>(from: (T, T), to: (T, T), v: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to.0 + (v - from.0) * (to.1 - to.0) / (from.1 - from.0)
}
