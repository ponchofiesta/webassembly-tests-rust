use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast, Clamped};
use wasm_bindgen::prelude::JsValue;

pub fn convolve(canvas: &HtmlCanvasElement, matrix: &Vec<Vec<f32>>, factor: f32) {

    let width = canvas.width() as usize;
    let height = canvas.height() as usize;

    web_sys::console::debug_1(&JsValue::from("Rust: 1"));
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    web_sys::console::debug_1(&JsValue::from("Rust: 2"));

    let data: Clamped<Vec<u8>> = context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap().data();
    web_sys::console::debug_1(&JsValue::from("Rust: 3"));
    let mut out = context.create_image_data_with_sw_and_sh(width as f64, height as f64).unwrap().data();

    web_sys::console::debug_1(&JsValue::from("Rust: 4"));
    let matrix_width = matrix[0].len();
    let matrix_height = matrix.len();
    let half = (matrix_height / 2) as usize;

    web_sys::console::debug_1(&JsValue::from(format!("Rust: helf={}", half)));
    web_sys::console::debug_1(&JsValue::from("Rust: 5"));
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let px = (y * width + x) * 4;
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;

            for cy in 0..matrix_width {
                for cx in 0..matrix_height {
                    let cpx = ((y + (cy - half)) * width + (x + (cx - half))) * 4;
                    if cpx > data.len() {
                        web_sys::console::debug_1(&JsValue::from(format!("Rust: cpx={} data_len={}", cpx, data.len())));
                    }
                    if cy > matrix.len() {
                        web_sys::console::debug_1(&JsValue::from("Rust: matrix"));
                    }
                    if cx > matrix[0].len() {
                        web_sys::console::debug_1(&JsValue::from("Rust: matrix[0]"));
                    }
                    r += data[cpx] as f32 * matrix[cy][cx];
//                    g += data[cpx + 1] as f32 * matrix[cy][cx];
//                    b += data[cpx + 2] as f32 * matrix[cy][cx];
                }
            }

//            out[px + 0] = (factor * r) as u8;
//            out[px + 1] = (factor * g) as u8;
//            out[px + 2] = (factor * b) as u8;
//            out[px + 3] = data[px + 3];
        }
    }
}