use crate::portfolio::{Portfolio, PortfolioAsset};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Portfolio {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(assets: Vec<PortfolioAsset>) -> Self {
        Portfolio::from(assets)
    }

    #[wasm_bindgen(js_name = "addAsset")]
    pub fn add_asset_wasm(&mut self, asset: PortfolioAsset) {
        self.add_asset(asset)
    }

    #[wasm_bindgen(getter = length)]
    pub fn len_wasm(&self) -> usize {
        self.size()
    }

    #[wasm_bindgen(js_name = "profitLoss")]
    pub fn profit_loss_wasm(&self) -> Option<f64> {
        self.profit_loss()
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
    pub fn init_wasm(
        // portfolio_weight: f64,
        name: String,
        quantity: f64,
        value_at_position_open: f64,
        values: Vec<f64>,
    ) -> Self {
        PortfolioAsset::new(
            // portfolio_weight,
            name, quantity, values,
        )
    }

    #[wasm_bindgen(js_name = "currentValue")]
    pub fn current_value_wasm(&self) -> f64 {
        self.current_value()
    }

    #[wasm_bindgen(js_name = "currentTotalValue")]
    pub fn current_total_value_wasm(&self) -> f64 {
        self.current_total_value()
    }

    #[wasm_bindgen(js_name = "profitLoss")]
    pub fn profit_loss_wasm(&self) -> Option<f64> {
        self.profit_loss()
    }
}
