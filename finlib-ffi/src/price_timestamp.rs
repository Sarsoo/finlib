use alloc::boxed::Box;
use chrono::DateTime;
use finlib::market_data::price_range::PriceTimestamp;
use finlib::price::Side;

#[no_mangle]
pub unsafe extern "C" fn price_timestamp_new(
    value: f64,
    side: Side,
    timestamp_millis: i64,
) -> *mut PriceTimestamp {
    Box::into_raw(Box::new(
        PriceTimestamp::builder()
            .side(side)
            .time(DateTime::from_timestamp_millis(timestamp_millis).unwrap())
            .value(value)
            .build(),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn price_timestamp_set_value(price: *mut PriceTimestamp, new_price: f64) {
    (&mut *price).value = new_price;
}

#[no_mangle]
pub unsafe extern "C" fn price_timestamp_destroy(asset: *mut PriceTimestamp) {
    if !asset.is_null() {
        drop(Box::from_raw(asset));
    }
}
