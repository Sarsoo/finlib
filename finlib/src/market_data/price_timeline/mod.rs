use crate::market_data::price_range::{PriceRangePair, PriceTimestamp};
use crate::market_data::TimeSpan;
use crate::price::{PricePair, Side};
use alloc::vec::Vec;
use chrono::{DateTime, Utc};

pub mod static_timeline;
mod varying_timeline;

pub trait PriceTimeline {
    type Key;
    type Value;

    fn new(scale: TimeSpan) -> Self;
    fn len(&self) -> usize;

    fn add_price(&mut self, price: PriceTimestamp) -> Result<(), ()>;
    fn add_price_pair(&mut self, price: PricePair, time: DateTime<Utc>) -> Result<(), ()>;
    fn add_price_range(&mut self, price: PriceRangePair) -> Result<(), ()>;

    fn price_range(&self, time: DateTime<Utc>) -> Result<PriceRangePair, ()>;
    fn price_ranges(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<PriceRangePair>, ()>;

    fn price_range_by(&self, from: DateTime<Utc>, by: TimeSpan) -> Result<Self::Value, ()>;
    fn price_ranges_by(&self, by: TimeSpan) -> Result<Vec<Self::Value>, ()>;
    fn price_ranges_by_between(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        by: TimeSpan,
    ) -> Result<Vec<Self::Value>, ()>;

    fn last(&self) -> Result<PriceRangePair, ()>;
    fn closing_prices(&self, side: Side) -> Result<Vec<f64>, ()>;
    fn clear(&mut self);
}
