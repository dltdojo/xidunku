use wasm_bindgen::prelude::*;
use geopattern::geo_pattern::GeoPattern;

// https://github.com/suyash/geopattern-rs/blob/master/src/lib.rs

#[wasm_bindgen]
pub fn geopattern_gen_minified_svg_string(s: &str) -> String {
   GeoPattern::new(s).build().unwrap().to_minified_svg().unwrap()
}

#[wasm_bindgen]
pub fn geopattern_gen_base64_svg_string(s: &str) -> String {
   GeoPattern::new(s).build().unwrap().to_base64().unwrap()
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn addbug(a: i32, b: i32) -> i32 {
    return a + b + 1;
}