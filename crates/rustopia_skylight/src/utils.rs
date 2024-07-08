use std::ops::{Add, Div, Mul, Sub};

use bevy::prelude::*;

pub fn map_range<T: Copy>(from: (T, T), to: (T, T), v: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to.0 + (v - from.0) * (to.1 - to.0) / (from.1 - from.0)
}

pub fn color_from_temperature(kelvin: f32) -> Color {
    let kelvin = kelvin.clamp(1000., 40000.);
    let temp = kelvin / 100.;

    let r = if temp <= 66. {
        1.
    } else {
        329.698727446 * (temp - 60.).powf(-0.1332047592) / 255.
    }
    .clamp(0., 1.);

    let g = if temp <= 66. {
        (99.4708025861 * temp.ln() - 161.1195681661) / 255.
    } else {
        288.1221695283 * (temp - 60.).powf(-0.0755148492) / 255.
    }
    .clamp(0., 1.);

    let b = if temp >= 66. {
        1.
    } else if temp <= 19. {
        0.
    } else {
        (138.5177312231 * (temp - 10.).ln() - 305.0447927307) / 255.
    }
    .clamp(0., 1.);

    return Color::srgb(r, g, b);
}
