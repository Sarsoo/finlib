use chrono::NaiveDate;
#[cfg(feature = "py")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CurvePoint {
    pub bid_rate: f64,
    pub offer_rate: f64,
    pub date: NaiveDate,
}
