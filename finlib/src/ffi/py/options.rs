use crate::derivatives::options::blackscholes::option_surface::{
    OptionSurfaceParameters, OptionsSurface,
};
use crate::derivatives::options::blackscholes::OptionVariables;
use crate::derivatives::options::strategy::component::OptionStrategyComponent;
use crate::derivatives::options::strategy::strategy::OptionStrategy;
use crate::derivatives::options::strategy::{IOptionStrategy, IOptionStrategyComponent};
use crate::derivatives::options::OptionType;
use crate::price::enums::Side;
use crate::price::payoff::{Payoff, Premium, Profit};
use pyo3::prelude::*;
use std::ops::Range;

#[pymethods]
impl OptionVariables {
    #[new]
    pub fn init(
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

#[pymethods]
impl OptionSurfaceParameters {
    #[new]
    pub fn init(
        underlying_price: (isize, isize),
        underlying_price_bounds: (f64, f64),
        strike_price: (isize, isize),
        strike_price_bounds: (f64, f64),
        volatility: (isize, isize),
        volatility_bounds: (f64, f64),
        risk_free_interest_rate: (isize, isize),
        risk_free_interest_rate_bounds: (f64, f64),
        dividend: (isize, isize),
        dividend_bounds: (f64, f64),
        time_to_expiration: (isize, isize),
        time_to_expiration_bounds: (f64, f64),
    ) -> Self {
        OptionSurfaceParameters::from(
            Range {
                start: underlying_price.0,
                end: underlying_price.1,
            },
            underlying_price_bounds,
            Range {
                start: strike_price.0,
                end: strike_price.1,
            },
            strike_price_bounds,
            Range {
                start: volatility.0,
                end: volatility.1,
            },
            volatility_bounds,
            Range {
                start: risk_free_interest_rate.0,
                end: risk_free_interest_rate.1,
            },
            risk_free_interest_rate_bounds,
            Range {
                start: dividend.0,
                end: dividend.1,
            },
            dividend_bounds,
            Range {
                start: time_to_expiration.0,
                end: time_to_expiration.1,
            },
            time_to_expiration_bounds,
        )
    }

    #[pyo3(name = "walk")]
    pub fn walk_py(&self) -> PyResult<OptionsSurface> {
        let c = self.clone();
        match c.walk() {
            Ok(s) => Ok(s),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to construct matrix",
            )),
        }
    }
}

#[pymethods]
impl OptionsSurface {
    pub fn __len__(&self) -> usize {
        self.len()
    }

    #[pyo3(name = "generate")]
    pub fn generate_py(&mut self) -> PyResult<()> {
        match self.generate() {
            Ok(_) => Ok(()),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to construct matrix",
            )),
        }
    }

    #[pyo3(name = "par_generate")]
    pub fn par_generate_py(&mut self) -> PyResult<()> {
        match self.par_generate() {
            Ok(_) => Ok(()),
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to construct matrix",
            )),
        }
    }
}

#[pymethods]
impl OptionStrategy {
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

    #[pyo3(name = "components")]
    pub fn components_py(&self) -> Vec<OptionStrategyComponent> {
        self.components()
            .into_iter()
            .map(|x| {
                let val = x.lock().unwrap();
                OptionStrategyComponent::from(
                    val.option_type(),
                    val.side(),
                    val.strike(),
                    val.premium(),
                )
            })
            .collect()
    }

    #[pyo3(name = "add_component")]
    pub fn add_component_py(&mut self, component: OptionStrategyComponent) {
        self.add_component(component);
    }
}

#[pymethods]
impl OptionStrategyComponent {
    #[new]
    pub fn init(option_type: OptionType, side: Side, strike: f64, premium: f64) -> Self {
        Self::from(option_type, side, strike, premium)
    }

    #[pyo3(name = "payoff")]
    pub fn payoff_py(&self, underlying: f64) -> f64 {
        self.payoff(underlying)
    }

    #[pyo3(name = "profit")]
    pub fn profit_py(&self, underlying: f64) -> f64 {
        self.profit(underlying)
    }

    #[pyo3(name = "will_be_exercised")]
    pub fn will_be_exercised_py(&self, underlying: f64) -> bool {
        self.will_be_exercised(underlying)
    }
}
