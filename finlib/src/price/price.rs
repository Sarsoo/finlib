use crate::price::enums::Side;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Price {
    pub value: f64,
    pub side: Side,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PricePair {
    /// buy price
    pub bid: f64,
    /// sell price
    pub offer: f64,
}

impl PricePair {
    pub fn from(bid: f64, offer: f64) -> Self {
        PricePair { bid, offer }
    }

    pub fn new() -> Self {
        PricePair { bid: 0., offer: 0. }
    }

    pub fn spread(&self) -> f64 {
        self.offer - self.bid
    }

    pub fn midpoint(&self) -> f64 {
        ((self.offer - self.bid) / 2.) + self.bid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midpoint() {
        let price = PricePair { bid: 5., offer: 7. };
        assert_eq!(price.midpoint(), 6.);
    }

    #[test]
    fn spread() {
        let price = PricePair { bid: 5., offer: 7. };
        assert_eq!(price.spread(), 2.);
    }
}
