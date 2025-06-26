use alloc::boxed::Box;
use finlib::market_data::price_range::PriceRange;

#[no_mangle]
pub unsafe extern "C" fn price_range_new(
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
) -> *mut PriceRange {
    Box::into_raw(Box::new(
        PriceRange::builder()
            .open(open)
            .close(close)
            .high(high)
            .low(low)
            .volume(volume)
            .build(),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn price_range_destroy(asset: *mut PriceRange) {
    if !asset.is_null() {
        drop(Box::from_raw(asset));
    }
}
