use crate::impl_premium_profit;
use crate::price::enums::Side;
use crate::price::payoff::{Payoff, Premium, Profit};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Swap {
    pub fixed_rate: f64,
    pub fixed_side: Side,
    pub premium: f64,
}

impl Swap {
    pub fn from(fixed_rate: f64, fixed_side: Side, premium: f64) -> Self {
        Self {
            fixed_rate,
            fixed_side,
            premium,
        }
    }

    pub fn from_pure(fixed_rate: f64, fixed_side: Side) -> Self {
        Self {
            fixed_rate,
            fixed_side,
            premium: 0.0,
        }
    }
}

impl Premium for Swap {
    fn premium(&self) -> f64 {
        self.premium
    }

    fn side(&self) -> Side {
        self.fixed_side
    }
}

impl Payoff<f64> for Swap {
    fn payoff(&self, underlying: f64) -> f64 {
        match self.fixed_side {
            Side::Buy => underlying - self.fixed_rate,
            Side::Sell => self.fixed_rate - underlying,
        }
    }
}

impl_premium_profit!(f64, Swap);

impl Payoff<Vec<f64>> for Swap {
    fn payoff(&self, underlying: Vec<f64>) -> f64 {
        let mut count = 0;
        let mut rate_sum = 0.;
        for i in underlying {
            count += 1;
            rate_sum += i;
        }

        let average_rate = rate_sum / count as f64;

        match self.fixed_side {
            Side::Buy => average_rate - self.fixed_rate,
            Side::Sell => self.fixed_rate - average_rate,
        }
    }
}

impl_premium_profit!(Vec<f64>, Swap);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::price::enums::Side::{Buy, Sell};

    #[test]
    fn buy() {
        let swap = Swap::from(100., Buy, 0.0);
        assert_eq!(swap.payoff(101.), 1.0);
    }

    #[test]
    fn sell() {
        let swap = Swap::from(100., Sell, 0.0);
        assert_eq!(swap.payoff(101.), -1.0);
    }

    #[test]
    fn buy_from_multiple() {
        let swap = Swap::from(100., Buy, 0.0);
        assert_eq!(swap.payoff(vec![100., 101., 102.]), 1.0);
    }

    #[test]
    fn sell_from_multiple() {
        let swap = Swap::from(100., Sell, 0.0);
        assert_eq!(swap.payoff(vec![100., 101., 102.]), -1.0);
    }

    #[test]
    fn buy_premium() {
        let swap = Swap::from(100., Buy, 5.0);
        assert_eq!(swap.profit(101.), -4.0);
    }

    #[test]
    fn sell_premium() {
        let swap = Swap::from(100., Sell, 5.0);
        assert_eq!(swap.profit(101.), 4.0);
    }
}
