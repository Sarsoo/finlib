use crate::market_data::price_range::{PriceRangePair, PriceTimestamp};
use crate::market_data::price_timeline::static_timeline::StaticPriceTimeline;
use crate::market_data::price_timeline::PriceTimeline;
use crate::price::{PricePair, Side};
use alloc::vec;
use alloc::vec::Vec;
use chrono::{DateTime, Utc};
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
type Map = HashMap<TimeSpan, StaticPriceTimeline>;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(not(feature = "std"))]
type Map = BTreeMap<TimeSpan, StaticPriceTimeline>;

use crate::market_data::TimeSpan;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

type KeyType = DateTime<Utc>;
type ValueType = PriceRangePair;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct VaryingPriceTimeline {
    timelines: Map,
}

impl VaryingPriceTimeline {
    pub fn new(scales: Vec<TimeSpan>) -> VaryingPriceTimeline {
        let mut map = Map::new();

        for s in scales {
            map.insert(s, StaticPriceTimeline::new(s));
        }

        VaryingPriceTimeline { timelines: map }
    }

    fn add_scale(&mut self, scale: TimeSpan) -> Result<(), ()> {
        if self.timelines.contains_key(&scale) {
            Err(())
        } else {
            self.timelines
                .insert(scale, StaticPriceTimeline::new(scale));
            Ok(())
        }
    }

    fn price_range_by(&self, time: DateTime<Utc>, scale: TimeSpan) -> Result<PriceRangePair, ()> {
        if let Some(timeline) = self.timelines.get(&scale) {
            timeline.price_range(time)
        } else {
            Err(())
        }
    }

    fn price_ranges_by(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        scale: TimeSpan,
    ) -> Result<Vec<PriceRangePair>, ()> {
        if let Some(timeline) = self.timelines.get(&scale) {
            timeline.price_ranges(from, to)
        } else {
            Err(())
        }
    }

    fn timeline_by(&self, scale: TimeSpan) -> Result<&StaticPriceTimeline, ()> {
        match self.timelines.get(&scale) {
            None => Err(()),
            Some(timeline) => Ok(timeline),
        }
    }

    fn timeline_by_mut(&mut self, scale: TimeSpan) -> Result<&mut StaticPriceTimeline, ()> {
        match self.timelines.get_mut(&scale) {
            None => Err(()),
            Some(timeline) => Ok(timeline),
        }
    }
}

impl PriceTimeline for VaryingPriceTimeline {
    type Key = KeyType;
    type Value = ValueType;

    fn new(scale: TimeSpan) -> Self {
        let mut map = Map::new();

        map.insert(scale, StaticPriceTimeline::new(scale));

        VaryingPriceTimeline { timelines: map }
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn add_price(&mut self, price: PriceTimestamp) -> Result<(), ()> {
        let mut result = vec![];
        for t in self.timelines.values_mut() {
            result.push(t.add_price(price));
        }

        if result.iter().any(|x| x.is_err()) {
            Err(())
        } else {
            Ok(())
        }
    }

    fn add_price_pair(&mut self, price: PricePair, time: DateTime<Utc>) -> Result<(), ()> {
        let mut result = vec![];
        for t in self.timelines.values_mut() {
            result.push(t.add_price_pair(price.clone(), time));
        }

        if result.iter().any(|x| x.is_err()) {
            Err(())
        } else {
            Ok(())
        }
    }

    fn add_price_range(&mut self, price: PriceRangePair) -> Result<(), ()> {
        let mut result = vec![];
        for t in self.timelines.values_mut() {
            result.push(t.add_price_range(price.clone()));
        }

        if result.iter().any(|x| x.is_err()) {
            Err(())
        } else {
            Ok(())
        }
    }

    fn price_range(&self, time: DateTime<Utc>) -> Result<PriceRangePair, ()> {
        for t in self.timelines.values() {
            match t.price_range(time) {
                Ok(r) => return Ok(r),
                Err(_) => {}
            }
        }
        Err(())
    }

    fn price_ranges(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<PriceRangePair>, ()> {
        for t in self.timelines.values() {
            match t.price_ranges(from, to) {
                Ok(r) => return Ok(r),
                Err(_) => {}
            }
        }
        Err(())
    }

    fn price_range_by(&self, from: DateTime<Utc>, by: TimeSpan) -> Result<Self::Value, ()> {
        if let Some(timeline) = self.timelines.get(&by) {
            timeline.price_range(from)
        } else {
            for (t, timeline) in self.timelines.iter() {
                if t < &by {
                    return timeline.price_range_by(from, by);
                }
            }
            Err(())
        }
    }

    fn price_ranges_by(&self, by: TimeSpan) -> Result<Vec<Self::Value>, ()> {
        if let Some(timeline) = self.timelines.get(&by) {
            Ok(timeline.iter().map(|(_, x)| x.clone()).collect())
        } else {
            for (t, timeline) in self.timelines.iter() {
                if t < &by {
                    return timeline.price_ranges_by(by);
                }
            }
            Err(())
        }
    }

    fn price_ranges_by_between(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        by: TimeSpan,
    ) -> Result<Vec<Self::Value>, ()> {
        if let Some(timeline) = self.timelines.get(&by) {
            timeline.price_ranges(from, to)
        } else {
            for (t, timeline) in self.timelines.iter() {
                if t < &by {
                    return timeline.price_ranges_by_between(from, to, by);
                }
            }
            Err(())
        }
    }

    fn last(&self) -> Result<PriceRangePair, ()> {
        for t in self.timelines.values() {
            match t.last() {
                Ok(r) => return Ok(r),
                Err(_) => {}
            }
        }
        Err(())
    }

    fn closing_prices(&self, side: Side) -> Result<Vec<f64>, ()> {
        todo!()
    }

    fn clear(&mut self) {
        self.timelines.clear();
    }
}
