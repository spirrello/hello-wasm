mod utils;

use wasm_bindgen::prelude::*;

// The #[wasm_bindgen] attribute indicates that the function below it will be accessible both in JavaScript and Rust.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// The #[wasm_bindgen] attribute indicates that the function below it will be accessible both in JavaScript and Rust.
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

// cfg_if! {
//     if #[cfg(feature = "wee_alloc")] {
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }
