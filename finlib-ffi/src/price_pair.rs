use alloc::boxed::Box;
use finlib::price::PricePair;

#[no_mangle]
pub unsafe extern "C" fn price_pair_new(bid: f64, offer: f64) -> *mut PricePair {
    Box::into_raw(Box::new(PricePair { bid, offer }))
}

#[no_mangle]
pub unsafe extern "C" fn price_pair_set_bid(price: *mut PricePair, new_price: f64) {
    (&mut *price).bid = new_price;
}

#[no_mangle]
pub unsafe extern "C" fn price_pair_set_offer(price: *mut PricePair, new_price: f64) {
    (&mut *price).offer = new_price;
}

#[no_mangle]
pub unsafe extern "C" fn price_pair_get_bid(price: *mut PricePair) -> f64 {
    (*price).bid
}

#[no_mangle]
pub unsafe extern "C" fn price_pair_get_offer(price: *mut PricePair) -> f64 {
    (*price).offer
}

#[no_mangle]
pub unsafe extern "C" fn price_pair_spread(price: *mut PricePair) -> f64 {
    (*price).spread()
}

#[no_mangle]
pub unsafe extern "C" fn price_pair_midpoint(price: *mut PricePair) -> f64 {
    (*price).midpoint()
}

#[no_mangle]
pub unsafe extern "C" fn price_pair_destroy(asset: *mut PricePair) {
    if !asset.is_null() {
        drop(Box::from_raw(asset));
    }
}
