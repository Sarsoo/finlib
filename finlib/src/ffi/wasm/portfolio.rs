use crate::derivatives::options::OptionContract;
use crate::market_data::price_range::{PriceRangePair, PriceTimestamp};
use crate::market_data::TimeSpan;
use crate::portfolio::strategy::{IStrategy, Strategy};
use crate::portfolio::{Portfolio, PortfolioAsset};
use crate::price::payoff::{Payoff, Profit};
use crate::price::PricePair;
use crate::risk::var::ValueAtRisk;
use crate::stats::{MuSigma, PopulationStats};
use chrono::{DateTime, Utc};
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
    pub fn profit_loss_wasm(&self) -> Result<f64, JsValue> {
        match self.profit_loss() {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from_str("ProfitLossError")),
        }
    }

    #[wasm_bindgen(js_name = "getAssetWeight")]
    pub fn get_asset_weight_wasm(&self) -> Vec<f64> {
        self.get_asset_weight().collect()
    }

    #[wasm_bindgen(js_name = "isValid")]
    pub fn is_valid_wasm(&self) -> bool {
        self.is_valid()
    }

    #[wasm_bindgen(js_name = "initialInvestment")]
    pub fn initial_investment_wasm(&self) -> Result<f64, JsValue> {
        match self.initial_investment() {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from_str("InitialInvestmentError")),
        }
    }

    #[wasm_bindgen(js_name = "addPrice")]
    pub fn add_price_wasm(&mut self, key: String, price: PriceTimestamp) -> Result<(), JsValue> {
        match self.add_price(key, price) {
            Ok(value) => Ok(()),
            Err(_) => Err(JsValue::from_str("InitialInvestmentError")),
        }
    }

    #[wasm_bindgen(js_name = "addPricePair")]
    pub fn add_price_pair_wasm(
        &mut self,
        key: String,
        price: PricePair,
        time: js_sys::Date,
    ) -> Result<(), JsValue> {
        let time = DateTime::<Utc>::from(&time);
        match self.add_price_pair(key, price, time) {
            Ok(value) => Ok(()),
            Err(_) => Err(JsValue::from_str("InitialInvestmentError")),
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
impl PortfolioAsset {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(name: String, quantity: f64, time_span: TimeSpan) -> Self {
        PortfolioAsset::new(name, quantity, time_span)
    }

    #[wasm_bindgen(js_name = "currentValue")]
    pub fn current_value_wasm(&self) -> Result<PriceRangePair, JsValue> {
        match self.current_value() {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from_str("currentValueError")),
        }
    }

    #[wasm_bindgen(js_name = "currentTotalValue")]
    pub fn current_total_value_wasm(&self) -> Result<f64, JsValue> {
        match self.current_total_value() {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from_str("currentValueError")),
        }
    }

    #[wasm_bindgen(js_name = "profitLoss")]
    pub fn profit_loss_wasm(&self) -> Result<f64, JsValue> {
        match self.profit_loss() {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from_str("ProfitLossError")),
        }
    }

    #[wasm_bindgen(js_name = "addPrice")]
    pub fn add_price_wasm(&mut self, price: PriceTimestamp) -> Result<(), JsValue> {
        match self.add_price(price) {
            Ok(value) => Ok(()),
            Err(_) => Err(JsValue::from_str("InitialInvestmentError")),
        }
    }

    #[wasm_bindgen(js_name = "addPricePair")]
    pub fn add_price_pair_wasm(
        &mut self,
        price: PricePair,
        time: js_sys::Date,
    ) -> Result<(), JsValue> {
        let time = DateTime::<Utc>::from(&time);
        match self.add_price_pair(price, time) {
            Ok(value) => Ok(()),
            Err(_) => Err(JsValue::from_str("InitialInvestmentError")),
        }
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
