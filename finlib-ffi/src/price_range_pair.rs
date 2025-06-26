use alloc::boxed::Box;
use chrono::DateTime;
use finlib::market_data::price_range::{PriceRange, PriceRangePair};

#[no_mangle]
pub unsafe extern "C" fn price_range_pair_new(
    open_timestamp_millis: i64,
    close_timestamp_millis: i64,
    bid: *mut PriceRange,
    offer: *mut PriceRange,
) -> *mut PriceRangePair {
    Box::into_raw(Box::new(
        PriceRangePair::builder()
            .open(DateTime::from_timestamp_millis(open_timestamp_millis).unwrap())
            .close(DateTime::from_timestamp_millis(close_timestamp_millis).unwrap())
            .bid((*bid).clone())
            .offer((*offer).clone())
            .build(),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn price_range_pair_destroy(asset: *mut PriceRangePair) {
    if !asset.is_null() {
        drop(Box::from_raw(asset));
    }
}
