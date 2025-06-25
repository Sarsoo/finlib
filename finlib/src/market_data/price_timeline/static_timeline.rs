use crate::market_data::price_range::{PriceRange, PriceRangePair, PriceTimestamp, TimeSpan};
use crate::price::{IPrice, PricePair, Side};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use chrono::{DateTime, Datelike, Duration, NaiveTime, Timelike, Utc, Weekday};

use crate::market_data::price_timeline::PriceTimeline;
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StaticPriceTimeline {
    raw_prices: BTreeMap<DateTime<Utc>, PriceRangePair>,
    scale: TimeSpan,
}

impl StaticPriceTimeline {
    fn merge_price(price: &mut PriceRangePair, merging: &PriceTimestamp) {
        match merging.side {
            Side::Buy => {
                if price.constituent_ticks == 0 {
                    price.bid.open = merging.value;
                }
                price.bid.close = merging.value;
                if merging.value < price.bid.low || price.constituent_ticks == 0 {
                    price.bid.low = merging.value;
                }
                if merging.value > price.bid.high {
                    price.bid.high = merging.value;
                }
            }
            Side::Sell => {
                if price.constituent_ticks == 0 {
                    price.offer.open = merging.value;
                }
                price.offer.close = merging.value;
                if merging.value < price.offer.low || price.constituent_ticks == 0 {
                    price.offer.low = merging.value;
                }
                if merging.value > price.offer.high {
                    price.offer.high = merging.value;
                }
            }
        }
        price.constituent_ticks += 1;
    }

    fn merge_price_pair(price: &mut PriceRangePair, merging: &PricePair) {
        if price.constituent_ticks == 0 {
            price.bid.open = merging.bid;
        }
        price.bid.close = merging.bid;
        if merging.bid < price.bid.low || price.constituent_ticks == 0 {
            price.bid.low = merging.bid;
        }
        if merging.bid > price.bid.high {
            price.bid.high = merging.bid;
        }

        if price.constituent_ticks == 0 {
            price.offer.open = merging.offer;
        }
        price.offer.close = merging.offer;
        if merging.offer < price.offer.low || price.constituent_ticks == 0 {
            price.offer.low = merging.offer;
        }
        if merging.offer > price.offer.high {
            price.offer.high = merging.offer;
        }
        price.constituent_ticks += 2;
    }

    fn new_price_range(time: DateTime<Utc>, duration: Duration) -> PriceRangePair {
        PriceRangePair::builder()
            .open(time)
            .close(time + duration)
            .bid(PriceRange::default())
            .offer(PriceRange::default())
            .build()
    }

    fn snap_time_to_scale(&self, time: DateTime<Utc>) -> DateTime<Utc> {
        match self.scale {
            TimeSpan::Second => time.with_nanosecond(0).unwrap(),
            TimeSpan::Minute => time.with_second(0).unwrap().with_nanosecond(0).unwrap(),
            TimeSpan::Hourly => time
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap(),
            TimeSpan::Daily => time
                .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .unwrap(),
            TimeSpan::Weekly => {
                let time = time - Duration::days(time.weekday().days_since(Weekday::Mon) as i64);

                assert_eq!(time.weekday(), Weekday::Mon);

                time.with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                    .unwrap()
            }
        }
    }

    fn scale_duration(&self) -> Duration {
        match self.scale {
            TimeSpan::Second => Duration::seconds(1),
            TimeSpan::Minute => Duration::minutes(1),
            TimeSpan::Hourly => Duration::hours(1),
            TimeSpan::Daily => Duration::days(1),
            TimeSpan::Weekly => Duration::weeks(1),
        }
    }
}

impl PriceTimeline for StaticPriceTimeline {
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
        let time = self.snap_time_to_scale(price.time()?);
        let duration = self.scale_duration();

        let current_range = self
            .raw_prices
            .entry(time)
            .or_insert(Self::new_price_range(time, duration));

        Self::merge_price(current_range, &price);

        Ok(())
    }

    fn add_price_pair(&mut self, price: PricePair, time: DateTime<Utc>) -> Result<(), ()> {
        let time = self.snap_time_to_scale(time);
        let duration = self.scale_duration();

        let current_range = self
            .raw_prices
            .entry(time)
            .or_insert(Self::new_price_range(time, duration));

        Self::merge_price_pair(current_range, &price);

        Ok(())
    }

    fn price_range(&self, time: DateTime<Utc>) -> Result<PriceRangePair, ()> {
        let time = self.snap_time_to_scale(time);

        match self.raw_prices.get(&time) {
            None => Err(()),
            Some(v) => Ok(v.clone()),
        }
    }

    fn price_ranges(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<PriceRangePair>, ()> {
        Ok(self
            .raw_prices
            .iter()
            .filter(|(d, p)| &from <= (*d) && (*d) < &to)
            .map(|(_, p)| p.clone())
            .collect())
    }

    fn last(&self) -> Result<PriceRangePair, ()> {
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

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn price_range_test() {
        let mut timeline = StaticPriceTimeline::new(TimeSpan::Second);

        let datetime = Utc.with_ymd_and_hms(2025, 06, 25, 10, 40, 2).unwrap();
        let datetime = datetime.with_nanosecond(100).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline.price_range(datetime).unwrap().constituent_ticks, 1);

        let datetime = datetime.with_nanosecond(2000).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline.price_range(datetime).unwrap().constituent_ticks, 2);

        let datetime = datetime.with_second(4).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        assert_eq!(timeline.len(), 2);
        assert_eq!(timeline.price_range(datetime).unwrap().constituent_ticks, 1);
    }

    #[test]
    fn price_range_daily_test() {
        let mut timeline = StaticPriceTimeline::new(TimeSpan::Daily);

        let datetime = Utc.with_ymd_and_hms(2025, 06, 25, 10, 40, 2).unwrap();
        let datetime = datetime.with_nanosecond(100).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline.price_range(datetime).unwrap().constituent_ticks, 1);

        let datetime = Utc.with_ymd_and_hms(2025, 06, 25, 12, 41, 2).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline.price_range(datetime).unwrap().constituent_ticks, 2);

        let datetime = Utc.with_ymd_and_hms(2025, 06, 28, 16, 20, 2).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        assert_eq!(timeline.len(), 2);
        assert_eq!(timeline.price_range(datetime).unwrap().constituent_ticks, 1);
    }

    #[test]
    fn price_ranges_test() {
        let mut timeline = StaticPriceTimeline::new(TimeSpan::Second);

        let datetime = Utc.with_ymd_and_hms(2025, 06, 25, 10, 40, 2).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        let datetime = datetime.with_second(3).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        let datetime = datetime.with_second(4).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        let datetime = datetime.with_second(5).unwrap();
        timeline
            .add_price(
                PriceTimestamp::builder()
                    .value(100.)
                    .side(Side::Buy)
                    .time(datetime)
                    .build(),
            )
            .unwrap();

        let ranges = timeline
            .price_ranges(
                Utc.with_ymd_and_hms(2025, 06, 25, 10, 40, 3).unwrap(),
                Utc.with_ymd_and_hms(2025, 06, 25, 10, 40, 5).unwrap(),
            )
            .unwrap();

        assert_eq!(ranges.len(), 2);
    }
}
