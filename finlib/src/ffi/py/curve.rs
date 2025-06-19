use crate::curve::curve::{Curve, CurveType};
use crate::curve::point::CurvePoint;
use crate::price::price::PricePair;
use pyo3::prelude::*;
use pyo3::types::PyDate;

#[pymethods]
impl Curve {
    #[new]
    pub fn init(curve_type: CurveType) -> Self {
        Self::new(curve_type)
    }

    #[pyo3(name = "size")]
    pub fn size_py(&mut self) -> PyResult<usize> {
        Ok(self.size())
    }

    pub fn __len__(&self) -> usize {
        self.size()
    }

    #[pyo3(name = "add_rate")]
    pub fn add_rate_py(&mut self, point: CurvePoint) -> PyResult<()> {
        self.add_rate(point);
        Ok(())
    }

    #[pyo3(name = "add_rate_from")]
    pub fn add_rate_from_py(
        &mut self,
        bid: f64,
        offer: f64,
        date: &Bound<'_, PyDate>,
    ) -> PyResult<()> {
        self.add_rate_from(bid, offer, date.extract().unwrap());
        Ok(())
    }

    #[pyo3(name = "get_cumulative_rate")]
    pub fn get_cumulative_rate_py(
        &mut self,
        at: &Bound<'_, PyDate>,
    ) -> PyResult<Option<PricePair>> {
        Ok(self.get_cumulative_rate(at.extract().unwrap()))
    }

    #[pyo3(name = "get_absolute_rate")]
    pub fn get_absolute_rate_py(&mut self, at: &Bound<'_, PyDate>) -> PyResult<Option<PricePair>> {
        Ok(self.get_absolute_rate(at.extract().unwrap()))
    }

    #[pyo3(name = "get_rate")]
    pub fn get_rate_py(&mut self, at: &Bound<'_, PyDate>) -> PyResult<Option<PricePair>> {
        Ok(self.get_rate(at.extract().unwrap()))
    }

    #[pyo3(name = "get_carry_rate")]
    pub fn get_carry_rate_py(
        &mut self,
        from: &Bound<'_, PyDate>,
        to: &Bound<'_, PyDate>,
    ) -> PyResult<Option<PricePair>> {
        Ok(self.get_carry_rate(from.extract().unwrap(), to.extract().unwrap()))
    }
}

#[pymethods]
impl CurvePoint {
    #[new]
    pub fn init(bid: f64, offer: f64, date: &Bound<'_, PyDate>) -> Self {
        Self {
            bid_rate: bid,
            offer_rate: offer,
            date: date.extract().unwrap(),
        }
    }
}
