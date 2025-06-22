use crate::derivatives::options::OptionContract;
use crate::portfolio::strategy::{IStrategy, Strategy};
use crate::portfolio::{Portfolio, PortfolioAsset};
use crate::price::payoff::{Payoff, Profit};
use crate::risk::var::ValueAtRisk;
use crate::stats::{MuSigma, PopulationStats};
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
    pub fn value_at_risk_pct_wasm(&mut self, confidence: f64) -> Result<f64, JsValue> {
        match self.value_at_risk_pct(confidence) {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Failed to calculate")),
        }
    }

    #[wasm_bindgen(js_name = "valueAtRisk")]
    pub fn value_at_risk_wasm(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
    ) -> Result<f64, JsValue> {
        match self.value_at_risk(confidence, initial_investment) {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Failed to calculate")),
        }
    }

    #[wasm_bindgen(js_name = "valueAtRiskAfterTime")]
    pub fn value_at_risk_afer_time_py_wasm(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
        at: isize,
    ) -> Result<f64, JsValue> {
        match self.value_at_risk_after_time(confidence, initial_investment, at) {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Failed to calculate")),
        }
    }
}

#[wasm_bindgen]
impl PortfolioAsset {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(name: String, quantity: f64, values: Vec<f64>) -> Self {
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

    #[wasm_bindgen(js_name = "applyRatesOfChange")]
    pub fn apply_rates_of_change_wasm(&mut self) {
        self.apply_rates_of_change();
    }

    #[wasm_bindgen(js_name = "meanAndStdDev")]
    pub fn mean_and_std_dev_wasm(&mut self) -> Result<MuSigma, JsValue> {
        match self.mean_and_std_dev() {
            Err(_) => Err(JsValue::from("Failed to calculate mean_and_std_dev")),
            Ok(m) => Ok(m),
        }
    }

    #[wasm_bindgen(js_name = "valueAtRiskPercent")]
    pub fn value_at_risk_pct_wasm(&mut self, confidence: f64) -> Result<f64, JsValue> {
        match self.value_at_risk_pct(confidence) {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Failed to calculate")),
        }
    }

    #[wasm_bindgen(js_name = "valueAtRisk")]
    pub fn value_at_risk_wasm(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
    ) -> Result<f64, JsValue> {
        match self.value_at_risk(confidence, initial_investment) {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Failed to calculate")),
        }
    }

    #[wasm_bindgen(js_name = "valueAtRiskAfterTime")]
    pub fn value_at_risk_afer_time_py_wasm(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
        at: isize,
    ) -> Result<f64, JsValue> {
        match self.value_at_risk_after_time(confidence, initial_investment, at) {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Failed to calculate")),
        }
    }
}

#[wasm_bindgen]
impl Strategy {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm() -> Self {
        Self::new()
    }

    #[wasm_bindgen(getter = length)]
    pub fn len_wasm(&self) -> usize {
        self.size()
    }

    #[wasm_bindgen(js_name = "payoff")]
    pub fn payoff_wasm(&mut self, underlying: f64) -> f64 {
        self.payoff(underlying)
    }

    #[wasm_bindgen(js_name = "profit")]
    pub fn profit_wasm(&mut self, underlying: f64) -> f64 {
        self.profit(underlying)
    }

    #[wasm_bindgen(js_name = "add_component")]
    pub fn add_component_wasm(&mut self, component: OptionContract) {
        self.add_component(component);
    }
}
