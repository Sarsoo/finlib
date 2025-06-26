use crate::price::{IPrice, PricePair, Side};
use bon::Builder;
use chrono::{DateTime, Utc};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

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

    fn side_value(&self, _: Side) -> Result<f64, ()> {
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

impl PriceRangePair {
    pub fn midpoint(&self) -> f64 {
        (self.offer.close - self.bid.close / 2.) + self.bid.close
    }

    pub fn spread(&self) -> f64 {
        self.offer.close - self.bid.close
    }

    pub fn merge_price(&mut self, merging: &PriceTimestamp) {
        match merging.side {
            Side::Buy => {
                if self.constituent_ticks == 0 {
                    self.bid.open = merging.value;
                }
                self.bid.close = merging.value;
                if merging.value < self.bid.low || self.constituent_ticks == 0 {
                    self.bid.low = merging.value;
                }
                if merging.value > self.bid.high {
                    self.bid.high = merging.value;
                }
            }
            Side::Sell => {
                if self.constituent_ticks == 0 {
                    self.offer.open = merging.value;
                }
                self.offer.close = merging.value;
                if merging.value < self.offer.low || self.constituent_ticks == 0 {
                    self.offer.low = merging.value;
                }
                if merging.value > self.offer.high {
                    self.offer.high = merging.value;
                }
            }
        }
        self.constituent_ticks += 1;
    }

    pub fn merge_price_pair(&mut self, merging: &PricePair) {
        if self.constituent_ticks == 0 {
            self.bid.open = merging.bid;
        }
        self.bid.close = merging.bid;
        if merging.bid < self.bid.low || self.constituent_ticks == 0 {
            self.bid.low = merging.bid;
        }
        if merging.bid > self.bid.high {
            self.bid.high = merging.bid;
        }

        if self.constituent_ticks == 0 {
            self.offer.open = merging.offer;
        }
        self.offer.close = merging.offer;
        if merging.offer < self.offer.low || self.constituent_ticks == 0 {
            self.offer.low = merging.offer;
        }
        if merging.offer > self.offer.high {
            self.offer.high = merging.offer;
        }
        self.constituent_ticks += 2;
    }
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

pub fn aggregate_ranges(vals: &[PriceRangePair]) -> PriceRangePair {
    let mut ret = PriceRangePair::builder()
        .open(DateTime::<Utc>::MAX_UTC)
        .close(DateTime::<Utc>::MIN_UTC)
        .bid(
            PriceRange::builder()
                .open(0.)
                .close(0.)
                .low(f64::MAX)
                .high(0.)
                .volume(0.)
                .build(),
        )
        .offer(
            PriceRange::builder()
                .open(0.)
                .close(0.)
                .low(f64::MAX)
                .high(0.)
                .volume(0.)
                .build(),
        )
        .build();

    for val in vals {
        if val.open < ret.open {
            ret.open = val.open;
            ret.bid.open = val.bid.open;
        }
        if val.close > ret.close {
            ret.close = val.close;
            ret.bid.close = val.bid.close;
        }

        if val.bid.low < ret.bid.low {
            ret.bid.low = val.bid.low;
        }
        if val.offer.low < ret.offer.low {
            ret.offer.low = val.offer.low;
        }

        if val.bid.high > ret.bid.high {
            ret.bid.high = val.bid.high;
        }
        if val.offer.high > ret.offer.high {
            ret.offer.high = val.offer.high;
        }

        ret.bid.volume += val.bid.volume;
        ret.offer.volume += val.offer.volume;
        ret.constituent_ticks += val.constituent_ticks;
    }

    ret
}
