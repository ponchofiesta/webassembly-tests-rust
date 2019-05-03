use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast, Clamped};
//use wasm_bindgen::prelude::JsValue;

pub fn convolve(canvas: &HtmlCanvasElement, matrix: &[f32], factor: f32) {
    let side = (matrix.len() as f32).sqrt() as usize;
    let half_side = (side as f32 / 2.0).floor() as usize;
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    let source = context.get_image_data(0.0, 0.0, canvas.width().into(), canvas.height().into()).unwrap();
    let source_data: Clamped<Vec<u8>> = source.data();
    let image_width = source.width() as usize;
    let image_height = source.height() as usize;
    let output = context.create_image_data_with_sw_and_sh(image_width as f64, image_height as f64).unwrap();
    let mut output_data = output.data();

    for y in 0..image_height {
        for x in 0..image_width {
            let output_index = (y * image_width + x) * 4;
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
                    if scy < image_height && scx < image_width {
                        let source_index = (scy * image_width + scx) * 4;
                        let modify = matrix[cy * side + cx];
                        r += source_data[source_index] as f32 * modify;
                        g += source_data[source_index + 1] as f32 * modify;
                        b += source_data[source_index + 2] as f32 * modify;
                    }
                }
            }
            output_data[output_index] = (r * factor) as u8;
            output_data[output_index + 1] = (g * factor) as u8;
            output_data[output_index + 2] = (b * factor) as u8;
            output_data[output_index + 3] = source_data[output_index + 3];
        }
    }

    context.put_image_data(&output, 0.0, 0.0).unwrap();
}
