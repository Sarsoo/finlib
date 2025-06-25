use crate::price::enums::Side;
use chrono::{DateTime, Utc};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub trait IPrice {
    fn value(&self) -> f64;
    fn side_value(&self, side: Side) -> Result<f64, ()>;
    fn time(&self) -> Result<DateTime<Utc>, ()>;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct Price {
    pub value: f64,
    pub side: Side,
}

impl IPrice for Price {
    fn value(&self) -> f64 {
        self.value
    }

    fn side_value(&self, side: Side) -> Result<f64, ()> {
        if side == self.side {
            return Ok(self.value());
        }
        Err(())
    }

    fn time(&self) -> Result<DateTime<Utc>, ()> {
        Err(())
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl IPrice for PricePair {
    fn value(&self) -> f64 {
        self.midpoint()
    }

    fn side_value(&self, side: Side) -> Result<f64, ()> {
        match side {
            Side::Buy => Ok(self.bid),
            Side::Sell => Ok(self.offer),
        }
    }

    fn time(&self) -> Result<DateTime<Utc>, ()> {
        Err(())
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
