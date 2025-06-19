use crate::price::enums::Side;
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

    pub fn net_return(&self, floating_rate: f64) -> f64 {
        match self.fixed_side {
            Side::Buy => floating_rate - self.fixed_rate - self.premium,
            Side::Sell => self.fixed_rate - floating_rate + self.premium,
        }
    }

    pub fn net_return_from_multiple(&self, floating_rate: impl IntoIterator<Item = f64>) -> f64 {
        let mut count = 0;
        let mut rate_sum = 0.;
        for i in floating_rate {
            count += 1;
            rate_sum += i;
        }

        let average_rate = rate_sum / count as f64;

        match self.fixed_side {
            Side::Buy => average_rate - self.fixed_rate - self.premium,
            Side::Sell => self.fixed_rate - average_rate + self.premium,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::price::enums::Side::{Buy, Sell};

    #[test]
    fn buy() {
        let swap = Swap::from(100., Buy, 0.0);
        assert_eq!(swap.net_return(101.), 1.0);
    }

    #[test]
    fn sell() {
        let swap = Swap::from(100., Sell, 0.0);
        assert_eq!(swap.net_return(101.), -1.0);
    }

    #[test]
    fn buy_from_multiple() {
        let swap = Swap::from(100., Buy, 0.0);
        assert_eq!(swap.net_return_from_multiple(vec![100., 101., 102.]), 1.0);
    }

    #[test]
    fn sell_from_multiple() {
        let swap = Swap::from(100., Sell, 0.0);
        assert_eq!(swap.net_return_from_multiple(vec![100., 101., 102.]), -1.0);
    }

    #[test]
    fn buy_premium() {
        let swap = Swap::from(100., Buy, 5.0);
        assert_eq!(swap.net_return(101.), -4.0);
    }

    #[test]
    fn sell_premium() {
        let swap = Swap::from(100., Sell, 5.0);
        assert_eq!(swap.net_return(101.), 4.0);
    }
}
