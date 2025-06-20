use crate::derivatives::swaps::Swap;
use crate::price::enums::Side;
use crate::price::payoff::Payoff;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Swap {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(fixed_rate: f64, fixed_side: Side, premium: f64) -> Self {
        Self::from(fixed_rate, fixed_side, premium)
    }

    #[wasm_bindgen(js_name = "payoff")]
    pub fn payoff_wasm(&self, underlying: f64) -> f64 {
        self.payoff(underlying)
    }

    #[wasm_bindgen(js_name = "payoffFromMultiple")]
    pub fn payoff_from_multiple_wasm(&self, underlying: Vec<f64>) -> f64 {
        self.payoff(underlying)
    }
}
