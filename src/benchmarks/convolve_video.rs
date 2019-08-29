use wasm_bindgen::Clamped;

pub fn convolve_video(mut data: Clamped<Vec<u8>>, width: usize, height: usize, matrix: [[f32; 3]; 3], factor: f32, count: usize) -> Clamped<Vec<u8>> {
    let side = matrix.len();
    let half = (side as f32 / 2.0).floor() as usize;
    for _i in 0..count {
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let index = (y * width + x) * 4;
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                for cy in 0..side {
                    for cx in 0..side {
                        let cpx = ((y + (cy - half)) * width + (x + (cx - half))) * 4;
                        r += data[cpx] as f32 * matrix[cy][cx];
                        g += data[cpx + 1] as f32 * matrix[cy][cx];
                        b += data[cpx + 2] as f32 * matrix[cy][cx];
                    }
                }
                data[index] = (factor * r) as u8;
                data[index + 1] = (factor * g) as u8;
                data[index + 2] = (factor * b) as u8;
            }
        }
    }
    return data;
}