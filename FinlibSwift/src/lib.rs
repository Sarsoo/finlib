use finlib;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Curve;
        type CurveType;
        type CurvePoint;
        type Portfolio;
        type PortfolioAsset;
        type Price;
        type PricePair;
        type Side;
        type Swap;
    }
}

use finlib::curve::curve::Curve;
use finlib::curve::curve::CurveType;
use finlib::curve::point::CurvePoint;
use finlib::price::enums::Side;
use finlib::price::price::Price;
use finlib::price::price::PricePair;
use finlib::risk::portfolio::Portfolio;
use finlib::risk::portfolio::PortfolioAsset;
use finlib::swaps::Swap;
