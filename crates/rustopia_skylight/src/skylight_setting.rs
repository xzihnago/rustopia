use bevy::prelude::*;

use crate::utils::*;

#[derive(Component)]
pub struct SkylightSetting {
    pub illuminance: f32,
    pub colortemp_min: f32,
    pub colortemp_max: f32,

    pub angvel: f32,
    pub inclination: f32,
    pub latitude: f32,
    pub hour_angle: f32,

    pub north: Vec3,
    pub east: Vec3,
    pub down: Vec3,
}

impl SkylightSetting {
    pub fn calc_skylight_data(&self) -> SkylightData {
        let up = Quat::from_axis_angle(self.east, self.latitude) * self.north;
        let direction = Quat::from_axis_angle(up, self.hour_angle)
            * Quat::from_axis_angle(self.east, self.latitude + self.inclination)
            * self.down;

        let ambient = map_range((-0.15, 0.), (10., 80.), direction.z.clamp(-0.15, 0.));
        let illuminance = self.illuminance * direction.z.max(0.);

        let kelvin = map_range(
            (0., 1.),
            (self.colortemp_min, self.colortemp_max),
            curve_from_height(direction.z),
        );
        let color = color_from_temperature(kelvin);

        SkylightData {
            up,
            direction,
            ambient,
            illuminance,
            color,
        }
    }
}

impl Default for SkylightSetting {
    fn default() -> Self {
        Self {
            illuminance: 100000.,
            colortemp_min: 1850.,
            colortemp_max: 6500.,

            angvel: 0.,
            inclination: 0.,
            latitude: 0.,
            hour_angle: 0.,

            north: Vec3::Y,
            east: Vec3::X,
            down: Vec3::NEG_Z,
        }
    }
}

#[derive(Debug)]
pub struct SkylightData {
    pub up: Vec3,
    pub direction: Vec3,
    pub ambient: f32,
    pub illuminance: f32,
    pub color: Color,
}
