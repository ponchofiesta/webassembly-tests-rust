extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate futures;
extern crate serde;
extern crate wasm_bindgen_futures;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod utils;
pub mod tests;
pub mod testdata;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use futures::{Future, future};
use js_sys::Promise;
use web_sys::{Response, console};
use tests::sort::User;

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
pub fn fibonacci(n: i32) -> i32 {
    web_sys::console::log_1(&JsValue::from("Rust: fibonacci"));
    tests::fibonacci::fibonacci(n)
}

#[wasm_bindgen]
pub fn hanoi(n: i32, from: &str, to: &str, via: &str) -> String {
    web_sys::console::log_1(&JsValue::from("Rust: hanoi"));
    let mut hanoi = tests::hanoi::Hanoi::new();
    hanoi.hanoi(n, from, to, via).into()
}

#[wasm_bindgen]
pub fn sort() {
    web_sys::console::log_1(&JsValue::from("Rust: sort"));
    let mut users = testdata::DATA_SORT.lock().unwrap();
    //let before = users.get(0).unwrap().clone();
    //console::log_1(&JsValue::from(before.name));
    tests::sort::sort(&mut users);
    //let after = users.get(0).unwrap().clone();
    //console::log_1(&JsValue::from(after.name));
}

#[wasm_bindgen]
pub fn prime(max: usize) {
    tests::prime::prime(max);
}

#[wasm_bindgen]
pub fn prepare_test_data(test: &str, data: JsValue) {
    web_sys::console::log_1(&JsValue::from("Rust: prepare_test_data"));
    match test {
        "sort" => {
            let data: Vec<User> = data.into_serde().unwrap();
            //console::log_1(&JsValue::from(format!("prepare_test_data_sort {}", data.len())));
            let mut entry = testdata::DATA_SORT_BASE.lock().unwrap();
            //console::log_1(&JsValue::from(format!("prepare_test_data_sort {}", entry.len())));
            *entry = data;
        },
        _ => panic!(format!("Rust: Invalid test type '{}'", test))
    }
}

//#[wasm_bindgen]
//pub fn prepare_test_data(test: &str, url: &str) -> Promise {
//    match test {
//        "sort" => prepare_test_data_sort(url),
//        _ => Promise::reject(&JsValue::from("Rust: Invalid test type specified."))
//    }
//}

#[wasm_bindgen]
pub fn reset_test_data(test: &str) {
    web_sys::console::log_1(&JsValue::from("Rust: reset_test_data"));
    match test {
        "sort" => {
            let base = testdata::DATA_SORT_BASE.lock().unwrap();
            //console::log_1(&JsValue::from(format!("Rust: reset_test_data base {:?}", &base[0])));
            let mut entry = testdata::DATA_SORT.lock().unwrap();
            *entry = base.clone();
            //console::log_1(&JsValue::from(format!("Rust: reset_test_data entry {:?}", &entry[0])));
        },
        _ => {}
    };
}

#[wasm_bindgen]
pub fn clear_test_data(test: &str) {
    web_sys::console::log_1(&JsValue::from("Rust: clear_test_data"));
    match test {
        "sort" => {
            let mut entry = testdata::DATA_SORT.lock().unwrap();
            entry.clear();
        },
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
