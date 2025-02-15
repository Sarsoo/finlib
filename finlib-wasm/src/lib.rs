use wasm_bindgen::prelude::wasm_bindgen;
use console_log;
use log::Level;

#[wasm_bindgen(start)]
fn start() {
    if let Err(_) = console_log::init_with_level(Level::Debug) {

    }
}

#[wasm_bindgen]
pub struct Interest { }

#[wasm_bindgen]
impl Interest {
    pub fn compound(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
        finlib::interest::compound(principal, rate, time, n)
    }
}

#[wasm_bindgen]
pub struct ValueAtRisk { }

#[wasm_bindgen]
impl ValueAtRisk {
    pub fn historical(values: Vec<f64>, confidence: f64) -> f64 {
        finlib::risk::var::historical::value_at_risk(&values, confidence)
    }

    pub fn varcovar(values: Vec<f64>, confidence: f64) -> f64 {
        finlib::risk::var::varcovar::value_at_risk(&values, confidence)
    }
}

#[wasm_bindgen]
pub struct Stats { }

#[wasm_bindgen]
impl Stats {
    pub fn covariance(slice: Vec<f64>, slice_two: Vec<f64>) -> Option<f64> {
        finlib::stats::covariance(&slice, &slice_two)
    }
}

#[wasm_bindgen]
pub struct Util { }

#[wasm_bindgen]
impl Util {
    pub fn rates_of_change(slice: Vec<f64>) -> Vec<f64> {
        finlib::util::roc::rates_of_change(&slice).collect::<Vec<_>>()
    }
}
