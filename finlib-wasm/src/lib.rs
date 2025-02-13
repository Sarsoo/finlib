use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn compound(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    finlib::interest::compound(principal, rate, time, n)
}

#[wasm_bindgen]
pub fn covariance(slice: Vec<f64>, slice_two: Vec<f64>) -> Option<f64> {
    finlib::stats::covariance(&slice, &slice_two)
}