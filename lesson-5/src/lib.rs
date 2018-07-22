#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
pub fn run() {
    log!("The {} is {}", "meaning of life", 42);
}
