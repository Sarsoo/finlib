#[no_mangle]
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    finlib::add(left, right)
}

#[no_mangle]
pub extern "C" fn interest_compound(principal: f32, rate: f32, time: f32, n: f32) -> f32 {
    finlib::interest::compound(principal, rate, time, n)
}