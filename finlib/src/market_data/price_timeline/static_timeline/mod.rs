#[cfg(test)]
mod test;

use crate::market_data::price_range::{
    aggregate_ranges, PriceRange, PriceRangePair, PriceTimestamp,
};
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
    fn new_price_range(time: KeyType, duration: Duration) -> ValueType {
        ValueType::builder()
            .open(time)
            .close(time + duration)
            .bid(PriceRange::default())
            .offer(PriceRange::default())
            .build()
    }

    fn iter(&self) -> impl Iterator<Item = (&KeyType, &ValueType)> {
        self.raw_prices.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&KeyType, &mut ValueType)> {
        self.raw_prices.iter_mut()
    }

    pub fn price_range_by(&self, from: KeyType, by: TimeSpan) -> Result<ValueType, ()> {
        let from = by.snap_back(from);
        let duration = by.duration();

        Ok(aggregate_ranges(
            &self
                .raw_prices
                .iter()
                .filter(|(d, _)| &from <= (*d) && (*d) < &(from + duration))
                .map(|(_, p)| p.clone())
                .collect::<Vec<ValueType>>(),
        ))
    }

    pub fn price_ranges_by(&self, by: TimeSpan) -> Result<Vec<ValueType>, ()> {
        let duration = by.duration();

        let mut ranges: Vec<Vec<ValueType>> = Vec::new();

        match (
            self.raw_prices.first_key_value(),
            self.raw_prices.last_key_value(),
        ) {
            (Some(f), Some(l)) => {
                let mut cursor = f.0.clone();

                let to = l.0.clone();
                while cursor + duration < to {
                    let window: Vec<ValueType> = self.range_slice(cursor, duration);

                    ranges.push(window);

                    cursor += duration;
                }

                Ok(ranges.into_iter().map(|v| aggregate_ranges(&v)).collect())
            }
            _ => Err(()),
        }
    }

    pub fn price_ranges_between_by(
        &self,
        from: KeyType,
        to: KeyType,
        by: TimeSpan,
    ) -> Result<Vec<ValueType>, ()> {
        let from = by.snap_back(from);
        let duration = by.duration();

        let mut ranges: Vec<Vec<ValueType>> = Vec::new();
        let mut cursor = from;
        while cursor + duration < to {
            let window: Vec<ValueType> = self.range_slice(cursor, duration);

            ranges.push(window);

            cursor += duration;
        }

        Ok(ranges.into_iter().map(|v| aggregate_ranges(&v)).collect())
    }

    fn range_slice(&self, cursor: DateTime<Utc>, duration: Duration) -> Vec<ValueType> {
        self.raw_prices
            .iter()
            .filter(|(d, _)| &cursor <= (*d) && (*d) < &(cursor + duration))
            .map(|(_, p)| p.clone())
            .collect()
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
