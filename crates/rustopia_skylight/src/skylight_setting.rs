use bevy::prelude::*;

use crate::{utils::*, SkylightData};

#[derive(Resource)]
pub struct SkylightSetting {
    pub illuminance: f32,
    pub brightness: f32,
    pub colortemp_min: f32,
    pub colortemp_max: f32,

    pub inclination: f32,
    pub latitude: f32,
    pub hour_angle: f32,
}

impl Default for SkylightSetting {
    fn default() -> Self {
        Self {
            illuminance: 100000.,
            brightness: 80.,
            colortemp_min: 1850.,
            colortemp_max: 6500.,

            inclination: 0.,
            latitude: 0.,
            hour_angle: 0.,
        }
    }
}

impl SkylightSetting {
    pub fn compute(&self) -> SkylightData {
        let up = Quat::from_axis_angle(Vec3::X, self.latitude) * Vec3::Y;
        let solar = Quat::from_axis_angle(up, self.hour_angle)
            * Quat::from_axis_angle(Vec3::X, self.latitude + self.inclination)
            * Vec3::NEG_Z;

        let ambient = map_range(
            solar.z.clamp(-0.15, 0.),
            (-0.15, 0.),
            (self.brightness.min(10.), self.brightness),
        );
        let directional = self.illuminance * solar.z.max(0.);

        let color = color_from_temperature(map_range(
            curve_from_height(solar.z),
            (0., 1.),
            (self.colortemp_min, self.colortemp_max),
        ));

        SkylightData {
            up,
            solar,
            ambient,
            directional,
            color,
        }
    }
}
