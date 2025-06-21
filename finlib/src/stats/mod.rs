mod covariance;
pub use covariance::*;

#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, get_all, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Copy, Debug, PartialEq, PartialOrd)]
pub struct MuSigma {
    pub mean: f64,
    pub std_dev: f64,
}

pub trait PopulationStats {
    fn mean_and_std_dev(&self) -> Result<MuSigma, ()>;
}

pub fn mean(slice: &[f64]) -> f64 {
    slice.iter().sum::<f64>() / slice.len() as f64
}

pub fn population_variance(slice: &[f64]) -> f64 {
    let mean = mean(slice);
    slice.iter().map(|x| f64::powi(x - mean, 2)).sum::<f64>() / slice.len() as f64
}

pub fn sample_variance(slice: &[f64]) -> f64 {
    let mean = mean(slice);
    slice.iter().map(|x| f64::powi(x - mean, 2)).sum::<f64>() / ((slice.len() - 1) as f64)
}

pub fn population_std_dev(slice: &[f64]) -> f64 {
    f64::sqrt(population_variance(slice))
}

pub fn sample_std_dev(slice: &[f64]) -> f64 {
    f64::sqrt(sample_variance(slice))
}
