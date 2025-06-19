use crate::risk::portfolio::{Portfolio, PortfolioAsset};
use pyo3::prelude::*;

#[pymethods]
impl Portfolio {
    #[new]
    pub fn init(assets: Vec<PortfolioAsset>) -> Self {
        Portfolio::from(assets)
    }

    #[pyo3(name = "is_valid")]
    pub fn is_valid_py(&self) -> bool {
        self.is_valid()
    }

    #[pyo3(name = "value_at_risk")]
    pub fn value_at_risk_py(
        &mut self,
        confidence: f64,
        initial_investment: f64,
    ) -> PyResult<Option<f64>> {
        Ok(self.value_at_risk(confidence, initial_investment))
    }

    #[pyo3(name = "value_at_risk_percent")]
    pub fn value_at_risk_pct_py(&mut self, confidence: f64) -> PyResult<Option<f64>> {
        Ok(self.value_at_risk_percent(confidence))
    }

    #[pyo3(name = "size")]
    pub fn size_py(&mut self) -> PyResult<usize> {
        Ok(self.size())
    }

    pub fn __len__(&self) -> usize {
        self.size()
    }
}

#[pymethods]
impl PortfolioAsset {
    #[new]
    pub fn init(portfolio_weight: f64, name: String, values: Vec<f64>) -> Self {
        PortfolioAsset::new(portfolio_weight, name, values)
    }
}
