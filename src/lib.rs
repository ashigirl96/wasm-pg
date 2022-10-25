mod utils;

use ndarray::array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let result = format!("Hello {}!", name);
    result
}

#[wasm_bindgen]
pub fn arr_sum() -> String {
    let mut arr = array![[1, 2, 3], [4, 5, 6]];
    let x: i32 = arr.sum();
    let result = format!("{}.sum() => {}", &arr, x);
    result
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}