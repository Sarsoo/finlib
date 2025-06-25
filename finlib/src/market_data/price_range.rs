use crate::price::{IPrice, Side};
use bon::Builder;
use chrono::{DateTime, Utc};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TimeSpan {
    Second,
    Minute,
    Hourly,
    Daily,
    Weekly,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Builder, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PriceTimestamp {
    pub value: f64,
    pub side: Side,
    time: DateTime<Utc>,
}

impl IPrice for PriceTimestamp {
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
        Ok(self.time)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Builder, Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct PriceRange {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

impl IPrice for PriceRange {
    fn value(&self) -> f64 {
        self.close
    }

    fn side_value(&self, side: Side) -> Result<f64, ()> {
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
#[derive(Builder, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PriceRangePair {
    open: DateTime<Utc>,
    close: DateTime<Utc>,

    pub bid: PriceRange,
    pub offer: PriceRange,
    #[builder(default = 0)]
    pub constituent_ticks: isize,
}

impl IPrice for PriceRangePair {
    fn value(&self) -> f64 {
        ((self.offer.value() - self.bid.value()) / 2.) + self.bid.value()
    }

    fn side_value(&self, side: Side) -> Result<f64, ()> {
        match side {
            Side::Buy => Ok(self.bid.value()),
            Side::Sell => Ok(self.offer.value()),
        }
    }

    fn time(&self) -> Result<DateTime<Utc>, ()> {
        Ok(self.close)
    }
}
