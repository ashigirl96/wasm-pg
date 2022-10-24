// //! Test suite for the Web and headless browsers.
//
// #![cfg(target_arch = "wasm32")]
//
// extern crate wasm_bindgen_test;
// use wasm_bindgen_test::*;
//
// wasm_bindgen_test_configure!(run_in_browser);
//
// #[wasm_bindgen_test]
// fn pass() {
//     assert_eq!(1 + 1, 2);
// }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}