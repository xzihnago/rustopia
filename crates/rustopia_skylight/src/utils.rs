use std::f32::consts::FRAC_2_PI;

use bevy::prelude::*;

pub fn curve_from_height(height: f32) -> f32 {
    let height = height.clamp(-1., 1.);

    match height {
        _ if height <= 0. => 0.,
        _ => -(height.acos() * FRAC_2_PI).powf(4.) + 1.,
    }
}

pub fn color_from_temperature(kelvin: f32) -> Color {
    let temp = kelvin.clamp(1000., 40000.) / 100.;

    let r = match temp {
        _ if temp <= 66. => 1.,
        _ => 1.2929 * (temp - 60.).powf(-0.133),
    }
    .clamp(0., 1.);

    let g = match temp {
        _ if temp <= 66. => 0.3901 * temp.ln() - 0.6318,
        _ => 1.1299 * (temp - 60.).powf(-0.076),
    }
    .clamp(0., 1.);

    let b = match temp {
        _ if temp >= 66. => 1.,
        _ if temp <= 19. => 0.,
        _ => 0.5432 * (temp - 10.).ln() - 1.1963,
    }
    .clamp(0., 1.);

    return Color::srgb(r, g, b);
}
