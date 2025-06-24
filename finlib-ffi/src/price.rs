use alloc::boxed::Box;
use finlib::price::enums::Side;
use finlib::price::price::{Price, PricePair};

#[no_mangle]
pub unsafe extern "C" fn price_new(price: f64, side: Side) -> *mut Price {
    Box::into_raw(Box::new(Price { value: price, side }))
}

#[no_mangle]
pub unsafe extern "C" fn price_set_val(price: *mut Price, new_price: f64) {
    (&mut *price).value = new_price;
}

#[no_mangle]
pub unsafe extern "C" fn price_set_side(price: *mut Price, new_side: Side) {
    (&mut *price).side = new_side;
}

#[no_mangle]
pub unsafe extern "C" fn price_get_val(price: *mut Price) -> f64 {
    (*price).value
}

#[no_mangle]
pub unsafe extern "C" fn price_get_side(price: *mut Price) -> Side {
    (*price).side
}

#[no_mangle]
pub unsafe extern "C" fn price_destroy(asset: *mut Price) {
    if !asset.is_null() {
        drop(Box::from_raw(asset));
    }
}

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
