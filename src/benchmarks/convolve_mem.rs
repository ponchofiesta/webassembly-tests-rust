pub fn convolve_mem(data: &[u8], width: usize, height: usize, matrix: &[f32], factor: f32) -> Vec<u8>{
    let side = (matrix.len() as f32).sqrt() as usize;
    let half_side = (side as f32 / 2.0).floor() as usize;
    let mut output_data= vec![0u8; width * height * 4];

    for y in 0..height {
        for x in 0..width {
            let output_index = (y * width + x) * 4;
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            for cy in 0..side {
                let scy = match (y + cy).checked_sub(half_side) {
                    Some(value) => value,
                    _ => continue
                };
                for cx in 0..side {
                    let scx = match (x + cx).checked_sub(half_side) {
                        Some(value) => value,
                        _ => continue
                    };
                    if scy < height && scx < width {
                        let source_index = (scy * width + scx) * 4;
                        let modify = matrix[cy * side + cx];
                        r += data[source_index] as f32 * modify;
                        g += data[source_index + 1] as f32 * modify;
                        b += data[source_index + 2] as f32 * modify;
                    }
                }
            }
            output_data[output_index] = (r * factor) as u8;
            output_data[output_index + 1] = (g * factor) as u8;
            output_data[output_index + 2] = (b * factor) as u8;
            output_data[output_index + 3] = data[output_index + 3];
        }
    }

    output_data
}
