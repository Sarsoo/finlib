use crate::options::blackscholes::option_surface::{OptionSurfaceParameters, OptionsSurface};
use crate::options::blackscholes::OptionVariables;
use std::ops::Range;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl OptionVariables {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(
        underlying_price: f64,
        strike_price: f64,
        volatility: f64,
        risk_free_interest_rate: f64,
        dividend: f64,
        time_to_expiration: f64,
    ) -> Self {
        OptionVariables::from(
            underlying_price,
            strike_price,
            volatility,
            risk_free_interest_rate,
            dividend,
            time_to_expiration,
        )
    }
}

#[wasm_bindgen]
impl OptionSurfaceParameters {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(
        underlying_price: Vec<isize>,
        underlying_price_bounds: Vec<f64>,
        strike_price: Vec<isize>,
        strike_price_bounds: Vec<f64>,
        volatility: Vec<isize>,
        volatility_bounds: Vec<f64>,
        risk_free_interest_rate: Vec<isize>,
        risk_free_interest_rate_bounds: Vec<f64>,
        dividend: Vec<isize>,
        dividend_bounds: Vec<f64>,
        time_to_expiration: Vec<isize>,
        time_to_expiration_bounds: Vec<f64>,
    ) -> Self {
        OptionSurfaceParameters::from(
            Range {
                start: underlying_price[0],
                end: underlying_price[1],
            },
            (underlying_price_bounds[0], underlying_price_bounds[1]),
            Range {
                start: strike_price[0],
                end: strike_price[1],
            },
            (strike_price_bounds[0], strike_price_bounds[1]),
            Range {
                start: volatility[0],
                end: volatility[1],
            },
            (volatility_bounds[0], volatility_bounds[1]),
            Range {
                start: risk_free_interest_rate[0],
                end: risk_free_interest_rate[1],
            },
            (
                risk_free_interest_rate_bounds[0],
                risk_free_interest_rate_bounds[1],
            ),
            Range {
                start: dividend[0],
                end: dividend[1],
            },
            (dividend_bounds[0], dividend_bounds[1]),
            Range {
                start: time_to_expiration[0],
                end: time_to_expiration[1],
            },
            (time_to_expiration_bounds[0], time_to_expiration_bounds[1]),
        )
    }

    #[wasm_bindgen(js_name = "walk")]
    pub fn walk_wasm(&self) -> Result<OptionsSurface, JsValue> {
        let c = self.clone();
        match c.walk() {
            Ok(s) => Ok(s),
            Err(_) => Err(JsValue::from("failed to construct matrix")),
        }
    }
}

#[wasm_bindgen]
impl OptionsSurface {
    #[wasm_bindgen(getter = length)]
    pub fn len_wasm(&self) -> usize {
        self.len()
    }

    #[wasm_bindgen(js_name = "generate")]
    pub fn generate_wasm(&mut self) -> Result<usize, JsValue> {
        match self.generate() {
            Ok(_) => Ok(self.len()),
            Err(_) => Err(JsValue::from("failed to construct matrix")),
        }
    }

    #[wasm_bindgen(js_name = "par_generate")]
    pub fn par_generate_wasm(&mut self) -> Result<usize, JsValue> {
        match self.par_generate() {
            Ok(_) => Ok(self.len()),
            Err(_) => Err(JsValue::from("failed to construct matrix")),
        }
    }
}
