use wasm_bindgen::prelude::*;
use crate::risk::portfolio::{Portfolio, PortfolioAsset};

#[wasm_bindgen]
impl Portfolio {

    #[wasm_bindgen(constructor)]
    pub fn init_wasm(assets: Vec<PortfolioAsset>) -> Self {
        Portfolio::from(assets)
    }

    #[wasm_bindgen(js_name = "isValid")]
    pub fn is_valid_wasm(&self) -> bool {
        self.is_valid()
    }

    #[wasm_bindgen(js_name = "valueAtRiskPercent")]
    pub fn value_at_risk_pct_wasm(&mut self, confidence: f64) -> Option<f64> {
        self.value_at_risk_percent(confidence)
    }

    #[wasm_bindgen(js_name = "valueAtRisk")]
    pub fn value_at_risk_wasm(&mut self, confidence: f64, initial_investment: f64) -> Option<f64> {
        self.value_at_risk(confidence, initial_investment)
    }
}

#[wasm_bindgen]
impl PortfolioAsset {

    #[wasm_bindgen(constructor)]
    pub fn init_wasm(portfolio_weight: f64, name: String, values: Vec<f64>) -> Self {
        PortfolioAsset::new(portfolio_weight, name, values)
    }
}