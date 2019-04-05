extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::mem::swap;

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
    if n < 2 {
        return n;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

#[wasm_bindgen]
pub fn hanoi(n: i32, from: i32, to: i32, via: i32) {
    if n > 0 {
        hanoi(n - 1, from, via, to);
        hanoi(n - 1, via, to, from);
    }
}
