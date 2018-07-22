#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[wasm_bindgen]
pub struct Image {
    pixels: Vec<Color>,
}

#[wasm_bindgen]
impl Image {
    pub fn new() -> Image {
        let color1 = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        let color2 = Color {
            red: 60,
            green: 70,
            blue: 90,
        };
        let pixels = vec![color1, color2];
        Image {
            pixels
        }
    }

    pub fn pixels_ptr(&self) -> *const Color {
        self.pixels.as_ptr()
    }
}
