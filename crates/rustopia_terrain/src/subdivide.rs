fn subdivide(map: Vec<f32>, (width, height): (usize, usize), subdivisions: usize) -> Vec<f32> {
    let mut new_map = Vec::with_capacity(width * height * subdivisions * subdivisions);

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let sw = width * y + x;
            let se = sw + 1;
            let nw = sw + width;
            let ne = nw + 1;
        }
    }

    new_map
}
