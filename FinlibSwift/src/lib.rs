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
        type OptionSurfaceParameters;
        type OptionsSurface;
    }
}

use finlib::curve::curve::Curve;
use finlib::curve::curve::CurveType;
use finlib::curve::point::CurvePoint;
use finlib::derivatives::options::blackscholes::option_surface::OptionSurfaceParameters;
use finlib::derivatives::options::blackscholes::option_surface::OptionsSurface;
use finlib::derivatives::swaps::Swap;
use finlib::portfolio::Portfolio;
use finlib::portfolio::PortfolioAsset;
use finlib::price::enums::Side;
use finlib::price::price::Price;
use finlib::price::price::PricePair;
