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
use wasm_bindgen_futures::JsFuture;
use futures::Future;
use web_sys::{Request, RequestInit, RequestMode, Response};

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
    tests::fibonacci::fibonacci(n)
}

#[wasm_bindgen]
pub fn hanoi(n: i32, from: &str, to: &str, via: &str) -> String {
    let mut hanoi = tests::hanoi::Hanoi::new();
    hanoi.hanoi(n, from, to, via).into()
}

#[wasm_bindgen]
pub fn sort() {
    //web_sys::console::log_1(data_path.into());

    let mut users = testdata::DATA_SORT_BASE.lock().unwrap();
    tests::sort::sort(&mut users);

}

pub fn prepare_test_data(test: &str, url: &str) {
    match test {
        "sort" => {
            fetch(url)
                .and_then(|response| {
                    let resp: Response = response.dyn_into().unwrap();
                    resp.json();
                })
                .and_then()
                .and_then(|json| JsFuture::from(json))
                .and_then(|json| {
                    let data = json.into_serde().unwrap();
                    let mut entry = testdata::DATA_SORT_BASE.lock().unwrap();
                    *entry = data;
                });
        }
        _ => {}
    }
}

fn fetch(url: &str) -> JsFuture {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    let future = JsFuture::from(request_promise);
    future
}
