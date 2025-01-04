use bevy::{
    image::ImageSampler,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub struct DebugTexture;

impl DebugTexture {
    pub fn checkerboard(width: usize, height: usize) -> Image {
        let mut data = vec![0; width * height * 4];

        for y in 0..height {
            for x in 0..width {
                let color = if (x ^ y) & 1 == 0 {
                    [0, 0, 0, 255]
                } else {
                    [255, 255, 255, 255]
                };

                let offset = ((y * width) + x) * 4;
                data[offset..offset + 4].copy_from_slice(&color);
            }
        }

        let mut image = Image::new(
            Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.sampler = ImageSampler::nearest();

        image
    }

    pub fn colorful() -> Image {
        const TEXTURE_SIZE: usize = 8;

        let mut palette = [
            255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102,
            255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
        ];

        let mut data = vec![0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

        for y in 0..TEXTURE_SIZE {
            let offset = TEXTURE_SIZE * y * 4;
            data[offset..offset + TEXTURE_SIZE * 4].copy_from_slice(&palette);
            palette.rotate_right(4);
        }

        Image::new(
            Extent3d {
                width: TEXTURE_SIZE as u32,
                height: TEXTURE_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        )
    }
}
