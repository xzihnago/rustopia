use std::{
    f32::consts::FRAC_2_PI,
    ops::{Add, Div, Mul, Sub},
};

use bevy::prelude::*;

pub fn map_range<T: Copy>(from: (T, T), to: (T, T), v: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to.0 + (v - from.0) * (to.1 - to.0) / (from.1 - from.0)
}

pub fn curve_from_height(height: f32) -> f32 {
    let height = height.clamp(-1., 1.).acos() * FRAC_2_PI;
    if height <= 0. {
        0.
    } else {
        -height.powf(4.) + 1.
    }
}

pub fn color_from_temperature(kelvin: f32) -> Color {
    let kelvin = kelvin.clamp(1000., 40000.);
    let temp = kelvin / 100.;

    let r = if temp <= 66. {
        1.
    } else {
        1.2929 * (temp - 60.).powf(-0.133)
    }
    .clamp(0., 1.);

    let g = if temp <= 66. {
        0.3901 * temp.ln() - 0.6318
    } else {
        1.1299 * (temp - 60.).powf(-0.076)
    }
    .clamp(0., 1.);

    let b = if temp >= 66. {
        1.
    } else if temp <= 19. {
        0.
    } else {
        0.5432 * (temp - 10.).ln() - 1.1963
    }
    .clamp(0., 1.);

    return Color::srgb(r, g, b);
}
