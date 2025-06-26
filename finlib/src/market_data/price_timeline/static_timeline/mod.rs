#[cfg(test)]
mod test;

use crate::market_data::price_range::{PriceRange, PriceRangePair, PriceTimestamp};
use crate::price::{IPrice, PricePair, Side};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use chrono::{DateTime, Duration, Utc};

use crate::market_data::price_timeline::PriceTimeline;
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
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StaticPriceTimeline {
    raw_prices: BTreeMap<KeyType, ValueType>,
    scale: TimeSpan,
}

impl StaticPriceTimeline {
    pub fn new_price_range(time: KeyType, duration: Duration) -> ValueType {
        ValueType::builder()
            .open(time)
            .close(time + duration)
            .bid(PriceRange::default())
            .offer(PriceRange::default())
            .build()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&KeyType, &ValueType)> {
        self.raw_prices.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&KeyType, &mut ValueType)> {
        self.raw_prices.iter_mut()
    }
}

impl IntoIterator for StaticPriceTimeline {
    type Item = (KeyType, ValueType);
    type IntoIter = <BTreeMap<KeyType, ValueType> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.raw_prices.into_iter()
    }
}

impl PriceTimeline for StaticPriceTimeline {
    type Key = KeyType;
    type Value = ValueType;

    fn new(scale: TimeSpan) -> Self {
        Self {
            raw_prices: BTreeMap::new(),
            scale,
        }
    }

    fn len(&self) -> usize {
        self.raw_prices.len()
    }

    fn add_price(&mut self, price: PriceTimestamp) -> Result<(), ()> {
        let time = self.scale.snap_back(price.time()?);
        let duration = self.scale.duration();

        let current_range = self
            .raw_prices
            .entry(time)
            .or_insert(Self::new_price_range(time, duration));

        current_range.merge_price(&price);

        Ok(())
    }

    fn add_price_pair(&mut self, price: PricePair, time: KeyType) -> Result<(), ()> {
        let time = self.scale.snap_back(time);
        let duration = self.scale.duration();

        let current_range = self
            .raw_prices
            .entry(time)
            .or_insert(Self::new_price_range(time, duration));

        current_range.merge_price_pair(&price);

        Ok(())
    }

    fn add_price_range(&mut self, price: PriceRangePair) -> Result<(), ()> {
        let time = self.scale.snap_back(price.time()?);
        let duration = self.scale.duration();

        let current_range = self
            .raw_prices
            .entry(time)
            .or_insert(Self::new_price_range(time, duration));

        current_range.merge_price_range(price);

        Ok(())
    }

    fn price_range(&self, time: KeyType) -> Result<ValueType, ()> {
        let time = self.scale.snap_back(time);

        match self.raw_prices.get(&time) {
            None => Err(()),
            Some(v) => Ok(v.clone()),
        }
    }

    fn price_ranges(&self, from: KeyType, to: KeyType) -> Result<Vec<ValueType>, ()> {
        Ok(self
            .raw_prices
            .iter()
            .filter(|(d, _)| &from <= (*d) && (*d) < &to)
            .map(|(_, p)| p.clone())
            .collect())
    }

    fn price_range_by(&self, from: KeyType, by: TimeSpan) -> Result<ValueType, ()> {
        if by < self.scale {
            return Err(());
        }

        let mut aggregated = StaticPriceTimeline::new(by);
        let to = from + by.duration();

        for i in self
            .raw_prices
            .values()
            .filter(|x| from < x.time().unwrap() && x.time().unwrap() < to)
        {
            aggregated.add_price_range(i.clone())?;
        }

        Ok(aggregated.price_range(from)?)
    }

    fn price_ranges_by(&self, by: TimeSpan) -> Result<Vec<ValueType>, ()> {
        if by < self.scale {
            return Err(());
        }

        let mut aggregated = StaticPriceTimeline::new(by);

        for i in self.raw_prices.values() {
            aggregated.add_price_range(i.clone())?;
        }

        Ok(aggregated.into_iter().map(|(_, x)| x).collect())
    }

    fn price_ranges_by_between(
        &self,
        from: KeyType,
        to: KeyType,
        by: TimeSpan,
    ) -> Result<Vec<ValueType>, ()> {
        if by < self.scale {
            return Err(());
        }

        let mut aggregated = StaticPriceTimeline::new(by);

        for i in self
            .raw_prices
            .values()
            .filter(|x| from < x.time().unwrap() && x.time().unwrap() < to)
        {
            aggregated.add_price_range(i.clone())?;
        }

        Ok(aggregated.into_iter().map(|(_, x)| x).collect())
    }

    fn last(&self) -> Result<ValueType, ()> {
        match self.raw_prices.last_key_value() {
            None => Err(()),
            Some((_, v)) => Ok(v.clone()),
        }
    }

    fn closing_prices(&self, side: Side) -> Result<Vec<f64>, ()> {
        match side {
            Side::Buy => Ok(self.raw_prices.values().map(|x| x.offer.close).collect()),
            Side::Sell => Ok(self.raw_prices.values().map(|x| x.bid.close).collect()),
        }
    }

    fn clear(&mut self) {
        self.raw_prices.clear();
    }
}
