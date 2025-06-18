use wasm_bindgen::prelude::*;
use crate::price::enums::Side;
use crate::price::price::{Price, PricePair};

#[wasm_bindgen]
impl Price {

    #[wasm_bindgen(constructor)]
    pub fn init_wasm(value: f64, side: Side) -> Self {
        Self {value, side}
    }
}

#[wasm_bindgen]
impl PricePair {

    #[wasm_bindgen(constructor)]
    pub fn init_wasm(bid: f64, offer: f64) -> Self {
        Self {bid, offer}
    }

    #[wasm_bindgen(js_name = "spread")]
    pub fn spread_wasm(&mut self) -> f64 {
        self.spread()
    }

    #[wasm_bindgen(js_name = "midpoint")]
    pub fn midpoint_wasm(&mut self) -> f64 {
        self.midpoint()
    }
}