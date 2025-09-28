mod utils;

use core::fmt;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let a = format!("Hello, {}!", name);
    alert(a.as_str());
}
