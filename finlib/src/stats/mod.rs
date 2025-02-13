mod covariance;
pub use covariance::*;

use rayon::prelude::*;

pub fn mean(slice: &[f64]) -> f64
{
    slice.par_iter().sum::<f64>() / slice.len() as f64
}

pub fn population_variance(slice: &[f64]) -> f64
{
    let mean = mean(slice);
    slice.par_iter()
        .map(|x| f64::powi(x - mean, 2))
        .sum::<f64>()
        / slice.len() as f64
}

pub fn sample_variance(slice: &[f64]) -> f64
{
    let mean = mean(slice);
    slice.par_iter()
        .map(|x| f64::powi(x - mean, 2))
        .sum::<f64>()
        / ((slice.len() - 1) as f64)
}

pub fn population_std_dev(slice: &[f64]) -> f64
{
    f64::sqrt(population_variance(slice))
}

pub fn sample_std_dev(slice: &[f64]) -> f64
{
    f64::sqrt(sample_variance(slice))
}