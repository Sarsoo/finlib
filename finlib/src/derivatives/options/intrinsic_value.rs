use crate::derivatives::options::Moneyness::{AtTheMoney, InTheMoney, OutOfTheMoney};
use crate::derivatives::options::{Moneyness, OptionContract, OptionType};

pub trait IntrinsicValue {
    fn intrinsic_value(&self, underlying_value: f64) -> f64;
    fn moneyness(&self, underlying_value: f64) -> Moneyness;
}

impl IntrinsicValue for OptionContract {
    fn intrinsic_value(&self, underlying_value: f64) -> f64 {
        match self.option_type {
            OptionType::Call => underlying_value - self.strike,
            OptionType::Put => self.strike - underlying_value,
        }
    }

    fn moneyness(&self, underlying_value: f64) -> Moneyness {
        let val = self.intrinsic_value(underlying_value);
        if val < 0. {
            OutOfTheMoney
        } else if val == 0. {
            AtTheMoney
        } else {
            InTheMoney
        }
    }
}
