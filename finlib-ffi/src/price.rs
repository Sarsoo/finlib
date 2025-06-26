use alloc::boxed::Box;
use finlib::price::enums::Side;
use finlib::price::price::Price;

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
