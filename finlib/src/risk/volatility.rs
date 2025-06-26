use crate::stats::sample_std_dev;
use crate::util::roc::rates_of_change;
use alloc::vec::Vec;
use num::Float;

pub trait Volatility {
    fn daily_volatility(&self) -> f64;
    fn annualised_volatility(&self) -> f64 {
        self.daily_volatility() * 252f64.sqrt()
    }
}

impl Volatility for &[f64] {
    fn daily_volatility(&self) -> f64 {
        sample_std_dev(rates_of_change(self).collect::<Vec<f64>>().as_slice())
    }
}
