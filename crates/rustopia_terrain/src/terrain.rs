use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

#[derive(Component)]
pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub altitude: Vec<f32>,
    pub physics: bool,
}

impl Terrain {
    pub fn random((width, height): (usize, usize), bound: f64) -> Self {
        let fbm = Fbm::<Perlin>::new(0);

        let noise_map = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(width, height)
            .set_x_bounds(-bound, bound)
            .set_y_bounds(-bound, bound)
            .build();

        let altitude = noise_map.iter().map(|x| (x + 1.) as f32 * 10.).collect();

        Self {
            width,
            height,
            altitude,
            physics: true,
        }
    }

    pub fn calc_mesh(&self) -> Mesh {
        let width = self.width;
        let height = self.height;

        // Position & UV
        let mut position = Vec::with_capacity(width * height);
        let mut uv = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let index = width * y + x;
                let height = self.altitude[index];
                position.push(Vec3::new(x as f32, y as f32, height as f32));

                uv.push([x as f32 / width as f32, y as f32 / height as f32]);
            }
        }

        // Indices
        let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);
        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let sw = width * y + x;
                let se = sw + 1;
                let nw = sw + width;
                let ne = nw + 1;

                if position[sw].z + position[ne].z < position[se].z + position[nw].z {
                    indices.push(sw);
                    indices.push(se);
                    indices.push(nw);

                    indices.push(se);
                    indices.push(ne);
                    indices.push(nw);
                } else {
                    indices.push(sw);
                    indices.push(se);
                    indices.push(ne);

                    indices.push(sw);
                    indices.push(ne);
                    indices.push(nw);
                }
            }
        }

        // Normals
        let mut normals = vec![Vec3::ZERO; width * height];
        for i in 0..(width - 1) * (height - 1) * 2 {
            let i1 = indices[i * 3];
            let i2 = indices[i * 3 + 1];
            let i3 = indices[i * 3 + 2];

            let normal = (position[i2] - position[i1])
                .cross(position[i3] - position[i1])
                .normalize();

            normals[i1] += normal;
            normals[i2] += normal;
            normals[i3] += normal;
        }
        for normal in &mut normals {
            *normal = normal.normalize();
        }

        let mut mesh: Mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, position);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        mesh.insert_indices(Indices::U32(indices.iter().map(|i| *i as u32).collect()));

        mesh
    }
}
