use pyo3::prelude::*;
use crate::price::enums::Side;
use crate::price::price::{Price, PricePair};

#[pymethods]
impl Price {

    #[new]
    pub fn init(value: f64, side: Side) -> Self {
        Self {value, side}
    }
}

#[pymethods]
impl PricePair {

    #[new]
    pub fn init(bid: f64, offer: f64) -> Self {
        Self {bid, offer}
    }

    #[pyo3(name = "spread")]
    pub fn spread_py(&mut self) -> PyResult<f64> {
        Ok(self.spread())
    }

    #[pyo3(name = "midpoint")]
    pub fn midpoint_py(&mut self) -> PyResult<f64> {
        Ok(self.midpoint())
    }
}
