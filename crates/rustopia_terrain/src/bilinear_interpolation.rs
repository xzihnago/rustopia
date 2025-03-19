use crate::terrain::Terrain;

impl Terrain {
    pub fn bilinear_interpolation(&self, x: f32, y: f32) -> BilinInterpData {
        let offset_x = x.fract();
        let offset_y = y.fract();

        let index = self.width * y as usize + x as usize;
        let sw = self.altitude[index];
        let se = self.altitude[index + 1];
        let nw = self.altitude[index + self.width];
        let ne = self.altitude[index + self.width + 1];

        let weight = Weight::new(offset_x, offset_y);
        let datum = Datum::new(sw, se, nw, ne);
        let slope = Slope::new(sw, se, nw, ne, offset_x, offset_y);
        let value = sw * weight.sw + se * weight.se + nw * weight.nw + ne * weight.ne;

        BilinInterpData {
            weight,
            datum,
            slope,
            value,
        }
    }
}

#[derive(Debug)]
pub struct BilinInterpData {
    pub weight: Weight,
    pub datum: Datum,
    pub slope: Slope,
    pub value: f32,
}

#[derive(Debug)]
pub struct Weight {
    pub sw: f32,
    pub se: f32,
    pub nw: f32,
    pub ne: f32,
}

impl Weight {
    pub fn new(offset_x: f32, offset_y: f32) -> Self {
        let offset_x_inv = 1. - offset_x;
        let offset_y_inv = 1. - offset_y;

        Self {
            sw: offset_x_inv * offset_y_inv,
            se: offset_x * offset_y_inv,
            nw: offset_x_inv * offset_y,
            ne: offset_x * offset_y,
        }
    }
}

#[derive(Debug)]
pub struct Datum {
    pub sw: f32,
    pub se: f32,
    pub nw: f32,
    pub ne: f32,
}

impl Datum {
    pub fn new(sw: f32, se: f32, nw: f32, ne: f32) -> Self {
        Self { sw, se, nw, ne }
    }
}

#[derive(Debug)]
pub struct Slope {
    pub x: f32,
    pub y: f32,
}

impl Slope {
    pub fn new(sw: f32, se: f32, nw: f32, ne: f32, offset_x: f32, offset_y: f32) -> Self {
        let offset_x_inv = 1. - offset_x;
        let offset_y_inv = 1. - offset_y;

        Self {
            x: (se - sw) * offset_y_inv + (ne - nw) * offset_y,
            y: (nw - sw) * offset_x_inv + (ne - se) * offset_x,
        }
    }
}
