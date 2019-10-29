extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate serde;
extern crate wasm_bindgen_futures;
extern crate sha2;
extern crate hex;
extern crate base64;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod utils;
pub mod benchmarks;
pub mod data;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use benchmarks::sort::User;
use wasm_bindgen::{Clamped, JsCast};
use js_sys::{Uint8Array};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}



#[wasm_bindgen]
pub fn base64(data: Vec<u8>) -> String {
    //web_sys::console::debug_1(&JsValue::from("Rust: iterate"));
    benchmarks::base64::base64(&data)
}

#[wasm_bindgen]
pub fn base64_prepared() -> String {
    //web_sys::console::debug_1(&JsValue::from("Rust: iterate"));
    let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::base64::base64(&data)
}

#[wasm_bindgen]
pub fn iterate(max: i32) -> i32 {
    //web_sys::console::debug_1(&JsValue::from("Rust: iterate"));
    benchmarks::iterate::iterate(max)
}

#[wasm_bindgen]
pub fn strings_dynamic(a: &str, b: &str) -> bool {
    //web_sys::console::debug_1(&JsValue::from("Rust: strings_dynamic"));
    benchmarks::strings::strings_dynamic(a, b)
}

#[wasm_bindgen]
pub fn strings_static(repeat: i32) {
    //web_sys::console::debug_1(&JsValue::from("Rust: strings_dynamic"));
    let a = "hello world";
    let b = "world";
    for _i in 0..repeat {
        benchmarks::strings::strings_dynamic(a, b);
    }
}

#[wasm_bindgen]
pub fn sum(repeat: i32) {
    //web_sys::console::debug_1(&JsValue::from("Rust: strings_dynamic"));
    for _i in 0..repeat {
        benchmarks::sum::sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    //web_sys::console::debug_1(&JsValue::from("Rust: fibonacci"));
    benchmarks::fibonacci::fibonacci(n)
}

#[wasm_bindgen]
pub fn hanoi(n: i32, from: &str, to: &str, via: &str) -> String {
    //web_sys::console::debug_1(&JsValue::from("Rust: hanoi"));
    let mut hanoi = benchmarks::hanoi::Hanoi::new();
    hanoi.hanoi(n, from, to, via).into()
}

#[wasm_bindgen]
pub fn sort() {
    //web_sys::console::debug_1(&JsValue::from("Rust: sort"));
    let mut users = data::DATA_SORT.lock().unwrap();
    benchmarks::sort::sort(&mut users);
}

#[wasm_bindgen]
pub fn prime(max: usize) -> Vec<usize> {
    //web_sys::console::debug_1(&JsValue::from("Rust: prime"));
    benchmarks::prime::prime(max)
}

#[wasm_bindgen]
pub fn sha256(data: &[u8]) -> String {
    //web_sys::console::debug_1(&JsValue::from("Rust: sort"));
    //let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::sha::sha256(&data)
}

#[wasm_bindgen]
pub fn sha256_prepared() -> String {
    //web_sys::console::debug_1(&JsValue::from("Rust: sort"));
    let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::sha::sha256(&data)
}

#[wasm_bindgen]
pub fn sha512(data: &[u8]) -> String {
    //web_sys::console::debug_1(&JsValue::from("Rust: sort"));
    //let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::sha::sha512(&data)
}

#[wasm_bindgen]
pub fn aes(data: &[u8]) -> Vec<u8> {
    //web_sys::console::debug_1(&JsValue::from("Rust: aes"));
    let key = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let iv = [17u8, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    //let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::aes::aes_encrypt(&key, &iv, &data)
}

#[wasm_bindgen]
pub fn deflate(data: &[u8]) -> Vec<u8> {
    //web_sys::console::debug_1(&JsValue::from("Rust: deflate"));
    //let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::deflate::deflate(&data)
}

#[wasm_bindgen]
pub fn deflate_prepared() -> Vec<u8> {
    //web_sys::console::debug_1(&JsValue::from("Rust: deflate"));
    let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::deflate::deflate(&data)
}

#[wasm_bindgen]
pub fn exif() -> String {
    //web_sys::console::debug_1(&JsValue::from("Rust: deflate"));
    let data = data::DATA_BYTES.lock().unwrap();
    benchmarks::exif::exif(&data)
}

#[wasm_bindgen]
pub fn convolve(canvas: &web_sys::HtmlCanvasElement) {
    //web_sys::console::debug_1(&JsValue::from("Rust: convolve"));
    let matrix = [
        0.0, 0.2, 0.0,
        0.2, 0.2, 0.2,
        0.0, 0.2, 0.0,
    ];
    benchmarks::convolve::convolve(&canvas, &matrix, 1.0);
}

#[wasm_bindgen]
pub fn convolve_mem(data: &[u8], width: usize, height: usize) -> Vec<u8> {
    //web_sys::console::debug_1(&JsValue::from("Rust: convolve"));
    let matrix = [
        0.0, 0.2, 0.0,
        0.2, 0.2, 0.2,
        0.0, 0.2, 0.0,
    ];
    benchmarks::convolve_mem::convolve_mem(data, width, height, &matrix, 1.0)
}

#[wasm_bindgen]
pub fn convolve_video(data: Clamped<Vec<u8>>, width: usize, height: usize, factor: f32, count: usize) -> Clamped<Vec<u8>> {
    //web_sys::console::debug_1(&JsValue::from("Rust: convolve_video"));
    let matrix: [[f32; 3]; 3] = [
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
    ];
    benchmarks::convolve_video::convolve_video(data, width, height, matrix, factor, count)
}

#[wasm_bindgen]
pub fn dom(n: usize) -> Result<(), JsValue> {
    //web_sys::console::debug_1(&JsValue::from("Rust: convolve_video"));
    benchmarks::dom::dom(n)
}

#[wasm_bindgen]
pub fn prepare_test_data(test: &str, data: JsValue) {
    web_sys::console::debug_1(&JsValue::from("Rust: prepare_test_data"));
    match test {
        "sort" => {
            let data: Vec<User> = data.into_serde().unwrap();
            let mut data_entry = data::DATA_SORT_BASE.lock().unwrap();
            *data_entry = data;
        },
        "bytes" => {
            //let arraybuffer: ArrayBuffer = data.dyn_into().unwrap();
            //let array: Uint8Array = Uint8Array::new(&arraybuffer);
            let uint8array: Uint8Array = data.dyn_into().unwrap();
            let mut data = vec![0u8; uint8array.byte_length() as usize];
            uint8array.copy_to(&mut data[..]);
            //web_sys::console::debug_1(&JsValue::from(format!("{} {}", &data[0], &data[1])));

            let mut data_entry = data::DATA_BYTES_BASE.lock().unwrap();
            *data_entry = data;
        }
        _ => panic!(format!("Rust: Invalid test type '{}'", test))
    }
}

#[wasm_bindgen]
pub fn reset_test_data(test: &str) {
    web_sys::console::debug_1(&JsValue::from("Rust: reset_test_data"));
    match test {
        "sort" => {
            let base = data::DATA_SORT_BASE.lock().unwrap();
            let mut data = data::DATA_SORT.lock().unwrap();
            *data = base.clone();
        },
        "bytes" => {
            let base = data::DATA_BYTES_BASE.lock().unwrap();
            let mut data = data::DATA_BYTES.lock().unwrap();
            *data = base.clone();
        }
        _ => {}
    };
}

#[wasm_bindgen]
pub fn clear_test_data(test: &str) {
    web_sys::console::debug_1(&JsValue::from("Rust: clear_test_data"));
    match test {
        "sort" => {
            let mut data = data::DATA_SORT.lock().unwrap();
            data.clear();
            let mut base = data::DATA_SORT_BASE.lock().unwrap();
            base.clear();
        },
        "bytes" => {
            let mut entry = data::DATA_BYTES.lock().unwrap();
            entry.clear();
            let mut entry = data::DATA_BYTES_BASE.lock().unwrap();
            entry.clear();
        }
        _ => {}
    };
}

//fn prepare_test_data_sort(url: &str) -> Promise {
//    let request_future = fetch(url)
//        .and_then(|response| response.dyn_into::<Response>().unwrap().json())
//        .and_then(|json: Promise| JsFuture::from(json))
//        .and_then(|json| {
//            //console::log_1(&json);
//            let data: Vec<User> = json.into_serde().unwrap();
//            console::log_1(&JsValue::from(format!("prepare_test_data_sort {}", data.len())));
//            let mut entry = testdata::DATA_SORT_BASE.lock().unwrap();
//            console::log_1(&JsValue::from(format!("prepare_test_data_sort {}", entry.len())));
//            *entry = data;
//            console::log_1(&JsValue::from(format!("prepare_test_data_sort {}", entry.len())));
//            future::ok(JsValue::TRUE)
//        });
//    future_to_promise(request_future)
//}
//
//fn fetch(url: &str) -> JsFuture {
//    let window = web_sys::window().unwrap();
//    let request_promise = window.fetch_with_str(url);
//    JsFuture::from(request_promise)
//}
