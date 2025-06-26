use super::*;
use chrono::{TimeZone, Timelike};

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
