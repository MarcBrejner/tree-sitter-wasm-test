use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn initialize(name: &str) {
    use tree_sitter::{Parser, Language};
    alert(&format!("Hello, {}!", name));
}