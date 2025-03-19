use bevy::prelude::*;

use crate::Droplet;
use crate::Terrain;

const EVAPORATION: f32 = 0.05;
const INERTIA: f32 = 0.1;
const CAPACITY: f32 = 1.0;
const DEPOSITION: f32 = 0.2;
const EROSION: f32 = 0.2;

impl Terrain {
    pub fn hydrolic_erosion(&mut self, mut droplet: Droplet) {
        let width = self.width as f32;
        let height = self.height as f32;

        while droplet.water > 0.05 {
            let origin = droplet.position;
            let origin_interp = self.bilinear_interpolation(origin.x, origin.y);

            // Update velocity
            let slope = Vec2::new(origin_interp.slope.x, origin_interp.slope.y);
            droplet.velocity = droplet.velocity * INERTIA - slope * (1. - INERTIA);

            // Move droplet
            let direction = droplet.velocity.normalize();
            droplet.position += direction;

            // Stop if not moving or drop out of world
            if droplet.velocity.length() < 0.01
                || droplet.position.x <= 0.
                || droplet.position.x >= width - 1.
                || droplet.position.y <= 0.
                || droplet.position.y >= height - 1.
            {
                let origin_index = self.width * origin.y as usize + origin.x as usize;

                self.altitude[origin_index] += droplet.sediment * origin_interp.weight.sw;
                self.altitude[origin_index + 1] += droplet.sediment * origin_interp.weight.se;
                self.altitude[origin_index + self.width] +=
                    droplet.sediment * origin_interp.weight.nw;
                self.altitude[origin_index + self.height + 1] +=
                    droplet.sediment * origin_interp.weight.ne;
                break;
            }

            let current_interp =
                self.bilinear_interpolation(droplet.position.x, droplet.position.y);
            let delta_h = current_interp.value - origin_interp.value;

            // Calculate sediment capacity
            let sediment_capacity =
                (CAPACITY * -delta_h * droplet.water * droplet.velocity.length()).max(0.01);

            if delta_h > 0. || droplet.sediment > sediment_capacity {
                let deposit = if delta_h > 0. {
                    delta_h.min(droplet.sediment)
                } else {
                    (droplet.sediment - sediment_capacity) * DEPOSITION
                };

                droplet.sediment -= deposit;

                // Sediment
                let origin_index = self.width * origin.y as usize + origin.x as usize;

                self.altitude[origin_index] += deposit * origin_interp.weight.sw;
                self.altitude[origin_index + 1] += deposit * origin_interp.weight.se;
                self.altitude[origin_index + self.width] += deposit * origin_interp.weight.nw;
                self.altitude[origin_index + self.width + 1] += deposit * origin_interp.weight.ne;
            } else {
                let erode = ((sediment_capacity - droplet.sediment) * EROSION).min(-delta_h);

                droplet.sediment += erode;

                // Erosion
                let origin_index = self.width * origin.y as usize + origin.x as usize;

                self.altitude[origin_index] -= erode * origin_interp.weight.sw;
                self.altitude[origin_index + 1] -= erode * origin_interp.weight.se;
                self.altitude[origin_index + self.width] -= erode * origin_interp.weight.nw;
                self.altitude[origin_index + self.width + 1] -= erode * origin_interp.weight.ne;

                // let mut change_height = |position: Vec2, delta: f32| {
                //     if position.x >= 0.
                //         && position.x < width - 1.
                //         && position.y >= 0.
                //         && position.y < height - 1.
                //     {
                //         let index = self.width * position.y as usize + position.x as usize;
                //         self.altitude[index] = (self.altitude[index] + delta).max(0.);
                //     }
                // };

                // let erode_radius = 3;

                // for y in -erode_radius..=erode_radius {
                //     for x in -erode_radius..=erode_radius {
                //         let position = origin + Vec2::new(x as f32, y as f32);
                //         let distance = (position - origin).length();

                //         if distance <= erode_radius as f32 {
                //             change_height(
                //                 position,
                //                 -erode * (1. - distance / erode_radius as f32).powi(2) / 10.,
                //             );
                //         }
                //     }
                // }
            }

            // Calculate new velocity
            droplet.velocity = Vec2::new(
                droplet.velocity.x.signum()
                    * (droplet.velocity.x * droplet.velocity.x - delta_h)
                        .max(0.)
                        .sqrt(),
                droplet.velocity.y.signum()
                    * (droplet.velocity.y * droplet.velocity.y - delta_h)
                        .max(0.)
                        .sqrt(),
            );

            droplet.water *= 1. - EVAPORATION;
        }
    }
}
