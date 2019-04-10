extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

#[macro_use]
extern crate serde_derive;

mod utils;
pub mod tests;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
//use tests::sort::User;

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

//#[wasm_bindgen]
//pub fn sort(data_path: &JsValue) {
//    //let persons: Vec<Person> = data.into_serde().unwrap();
//    web_sys::console::log_1(data_path.into());
//
//    let mut users: Vec<User> = reqwest::get(&data_path).json();
//
//
//    //console::log1(data.capacity());
//    //let data: [Person] = js_data.into_serde().unwrap();
//    tests::sort::sort(&mut users);
//
//    //let data = tests::sort::sort(&data);
//    //JsValue::from_serde(&data).unwrap()
//}
