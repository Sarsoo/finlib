use crate::derivatives::options::OptionContract;
use crate::portfolio::strategy::{IStrategy, Strategy};
use crate::portfolio::{Portfolio, PortfolioAsset};
use crate::price::payoff::{Payoff, Profit};
use crate::risk::var::ValueAtRisk;
use crate::stats::{MuSigma, PopulationStats};
use pyo3::prelude::*;

#[pymethods]
impl Portfolio {
    #[new]
    pub fn init(assets: Vec<PortfolioAsset>) -> Self {
        Portfolio::from(assets)
    }

    #[pyo3(name = "add_asset")]
    pub fn add_asset_py(&mut self, asset: PortfolioAsset) {
        self.add_asset(asset);
    }

    pub fn __len__(&self) -> usize {
        self.size()
    }

    #[pyo3(name = "profit_loss")]
    pub fn profit_loss_py(&self) -> Option<f64> {
        self.profit_loss()
    }

    #[pyo3(name = "get_asset_weight")]
    pub fn get_asset_weight_py(&self) -> Vec<f64> {
        self.get_asset_weight().collect()
    }

    #[pyo3(name = "apply_rates_of_change")]
    pub fn apply_rates_of_change_py(&mut self) {
        self.apply_rates_of_change();
    }

    #[pyo3(name = "valid_sizes")]
    pub fn valid_sizes_py(&self) -> bool {
        self.valid_sizes()
    }

    #[pyo3(name = "is_valid")]
    pub fn is_valid_py(&self) -> bool {
        self.is_valid()
    }

    #[pyo3(name = "get_mean_and_std")]
    pub fn get_mean_and_std_py(&mut self) -> PyResult<(f64, f64)> {
        match self.mean_and_std_dev() {
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "failed to calculate mean and std",
            )),
            Ok(MuSigma { mean, std_dev }) => Ok((mean, std_dev)),
        }
    }

    #[pyo3(name = "value_at_risk")]
    pub fn value_at_risk_py(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
    ) -> PyResult<Option<f64>> {
        match self.value_at_risk(confidence, initial_investment) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to calculate",
            )),
        }
    }

    #[pyo3(name = "value_at_risk_pct")]
    pub fn value_at_risk_pct_py(&mut self, confidence: f64) -> PyResult<f64> {
        match self.value_at_risk_pct(confidence) {
            Ok(value) => Ok(value),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to calculate",
            )),
        }
    }

    #[pyo3(name = "value_at_risk_after_time")]
    pub fn value_at_risk_after_time_py(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
        at: isize,
    ) -> PyResult<f64> {
        match self.value_at_risk_after_time(confidence, initial_investment, at) {
            Ok(value) => Ok(value),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to calculate",
            )),
        }
    }
}

#[pymethods]
impl PortfolioAsset {
    #[new]
    pub fn init(name: String, quantity: f64, values: Vec<f64>) -> Self {
        PortfolioAsset::new(name, quantity, values)
    }

    #[pyo3(name = "current_value")]
    pub fn current_value_py(&mut self) -> f64 {
        self.current_value()
    }

    #[pyo3(name = "current_total_value")]
    pub fn current_total_value_py(&mut self) -> f64 {
        self.current_total_value()
    }

    #[pyo3(name = "profit_loss")]
    pub fn profit_loss_py(&mut self) -> Option<f64> {
        self.profit_loss()
    }

    #[pyo3(name = "apply_rates_of_change")]
    pub fn apply_rates_of_change_py(&mut self) {
        self.apply_rates_of_change();
    }

    #[pyo3(name = "mean_and_std_dev")]
    pub fn mean_and_std_dev_py(&mut self) -> PyResult<MuSigma> {
        match self.mean_and_std_dev() {
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "failed to calculate mean and std",
            )),
            Ok(m) => Ok(m),
        }
    }

    #[pyo3(name = "value_at_risk")]
    pub fn value_at_risk_py(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
    ) -> PyResult<Option<f64>> {
        match self.value_at_risk(confidence, initial_investment) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to calculate",
            )),
        }
    }

    #[pyo3(name = "value_at_risk_pct")]
    pub fn value_at_risk_pct_py(&mut self, confidence: f64) -> PyResult<f64> {
        match self.value_at_risk_pct(confidence) {
            Ok(value) => Ok(value),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to calculate",
            )),
        }
    }

    #[pyo3(name = "value_at_risk_after_time")]
    pub fn value_at_risk_after_time_py(
        &mut self,
        confidence: f64,
        initial_investment: Option<f64>,
        at: isize,
    ) -> PyResult<f64> {
        match self.value_at_risk_after_time(confidence, initial_investment, at) {
            Ok(value) => Ok(value),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to calculate",
            )),
        }
    }
}

#[pymethods]
impl Strategy {
    #[new]
    pub fn init() -> Self {
        Self::new()
    }

    pub fn __len__(&self) -> usize {
        self.size()
    }

    #[pyo3(name = "payoff")]
    pub fn payoff_py(&self, underlying: f64) -> f64 {
        self.payoff(underlying)
    }

    #[pyo3(name = "profit")]
    pub fn profit_py(&self, underlying: f64) -> f64 {
        self.profit(underlying)
    }

    #[pyo3(name = "add_component")]
    pub fn add_component_py(&mut self, component: OptionContract) {
        self.add_component(component);
    }
}
