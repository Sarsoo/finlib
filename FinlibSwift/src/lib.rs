use finlib;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type PricePair;

        #[swift_bridge(init)]
        fn new() -> PricePair;

        #[swift_bridge(get(bid))]
        fn bid(&self) -> f64;
        #[swift_bridge(get(offer))]
        fn offer(&self) -> f64;

        fn set_offer(&self, offer: f64) {
            self.offer = offer;
        }

        fn spread(&self) -> f64;
    }
}

use finlib::price::PricePair;
