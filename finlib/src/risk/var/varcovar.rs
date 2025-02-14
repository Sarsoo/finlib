use crate::util::roc::rates_of_change;
use crate::stats;

#[cfg(feature = "parallel")]
use rayon::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal};

// https://medium.com/@serdarilarslan/value-at-risk-var-and-its-implementation-in-python-5c9150f73b0e

pub fn value_at_risk(values: &[f64], confidence: f64) -> f64 {
    let roc = rates_of_change(values).collect::<Vec<_>>();

    let mean = stats::mean(&roc);
    let std_dev = stats::sample_std_dev(&roc);

    let n = Normal::new(0.0, 1.0).unwrap();

    mean + std_dev * n.inverse_cdf(confidence)
}