use std::f32::consts::FRAC_2_PI;

use bevy::prelude::*;
use colortemp::temp_to_rgb;

use crate::SkylightSetting;

impl SkylightSetting {
    pub fn calc_skylight_data(&self) -> SkylightData {
        let axis = Quat::from_rotation_x(self.latitude) * Vec3::Y;

        let solar = Quat::from_axis_angle(axis, self.hour_angle)
            * Quat::from_rotation_x(self.latitude + self.inclination)
            * Vec3::NEG_Z;

        let illuminance = self.illuminance * solar.z.max(0.);

        let curve = (-(solar.z.acos() * FRAC_2_PI).powf(4.) + 1.).max(0.); // y = -x^4 + 1
        let kelvin = self.colortemp_interval * curve + self.colortemp_base;
        let colortemp = temp_to_rgb(kelvin as i64);
        let color = Color::rgb(
            colortemp.r as f32 / 255.,
            colortemp.g as f32 / 255.,
            colortemp.b as f32 / 255.,
        );

        SkylightData {
            axis,
            illuminance,
            solar,
            color,
        }
    }
}

#[derive(Debug)]
pub struct SkylightData {
    pub axis: Vec3,
    pub solar: Vec3,
    pub illuminance: f32,
    pub color: Color,
}
