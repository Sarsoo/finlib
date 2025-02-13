use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    finlib::add(left, right)
}

#[wasm_bindgen]
pub fn compound(principal: f32, rate: f32, time: f32, n: f32) -> f32 {
    finlib::interest::compound(principal, rate, time, n)
}