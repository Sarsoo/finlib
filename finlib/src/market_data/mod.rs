pub mod price_range;
pub mod price_timeline;

use chrono::{DateTime, Datelike, Duration, NaiveTime, Timelike, Utc, Weekday};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
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

impl TimeSpan {
    pub fn duration(&self) -> Duration {
        match self {
            TimeSpan::Second => Duration::seconds(1),
            TimeSpan::Minute => Duration::minutes(1),
            TimeSpan::Hourly => Duration::hours(1),
            TimeSpan::Daily => Duration::days(1),
            TimeSpan::Weekly => Duration::weeks(1),
        }
    }

    pub fn snap_back(&self, time: DateTime<Utc>) -> DateTime<Utc> {
        match self {
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

    pub fn snap_forward(&self, time: DateTime<Utc>) -> DateTime<Utc> {
        let backward = self.snap_back(time);
        let duration = self.duration();

        backward + duration
    }
}
