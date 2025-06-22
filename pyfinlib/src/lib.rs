use pyo3::prelude::*;

#[pymodule]
mod pyfinlib {
    use super::*;

    #[pymodule_export]
    use finlib::curve::curve::Curve;
    #[pymodule_export]
    use finlib::curve::curve::CurveType;
    #[pymodule_export]
    use finlib::curve::point::CurvePoint;

    #[pymodule_export]
    use finlib::portfolio::strategy::Strategy;
    #[pymodule_export]
    use finlib::portfolio::Portfolio;
    #[pymodule_export]
    use finlib::portfolio::PortfolioAsset;

    #[pymodule_export]
    use finlib::price::price::Price;
    #[pymodule_export]
    use finlib::price::price::PricePair;

    #[pymodule_export]
    use finlib::price::enums::Side;

    #[pymodule_export]
    use finlib::derivatives::swaps::Swap;

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();
        Ok(())
    }

    #[pymodule]
    mod indicators {
        use super::*;

        #[pymodule]
        mod rsi {
            use super::*;

            #[pymodule_export]
            use finlib::indicators::rsi::relative_strength_indicator;
            #[pymodule_export]
            use finlib::indicators::rsi::relative_strength_indicator_smoothed;
        }
    }

    #[pymodule]
    mod interest {
        use super::*;

        #[pyfunction]
        pub fn compound(principal: f64, rate: f64, time: f64, n: f64) -> PyResult<f64> {
            Ok(finlib::interest::compound(principal, rate, time, n))
        }
    }

    #[pymodule]
    mod options {
        use super::*;

        #[pymodule_export]
        use finlib::derivatives::options::blackscholes::option_surface::OptionSurfaceParameters;
        #[pymodule_export]
        use finlib::derivatives::options::blackscholes::option_surface::OptionsSurface;
        #[pymodule_export]
        use finlib::derivatives::options::blackscholes::OptionVariables;
        #[pymodule_export]
        use finlib::derivatives::options::OptionContract;
        #[pymodule_export]
        use finlib::derivatives::options::OptionGreeks;

        #[pymodule]
        mod strategy {
            use super::*;

            #[pymodule_export]
            use finlib::derivatives::options::templates::bear_put_spread;
            #[pymodule_export]
            use finlib::derivatives::options::templates::bull_call_spread;
            #[pymodule_export]
            use finlib::derivatives::options::templates::collar;
            #[pymodule_export]
            use finlib::derivatives::options::templates::iron_butterfly_spread;
            #[pymodule_export]
            use finlib::derivatives::options::templates::long_call_butterfly_spread;
            #[pymodule_export]
            use finlib::derivatives::options::templates::long_put_butterfly_spread;
            #[pymodule_export]
            use finlib::derivatives::options::templates::long_straddle;
            #[pymodule_export]
            use finlib::derivatives::options::templates::long_strangle;
            #[pymodule_export]
            use finlib::derivatives::options::templates::reverse_iron_butterfly_spread;
            #[pymodule_export]
            use finlib::derivatives::options::templates::short_call_butterfly_spread;
            #[pymodule_export]
            use finlib::derivatives::options::templates::short_put_butterfly_spread;
        }
    }

    #[pymodule]
    mod price {
        use super::*;

        #[pyfunction]
        pub fn calculate_bbo(vals: Vec<Price>) -> PyResult<PricePair> {
            Ok(finlib::price::bbo::calculate_bbo(vals))
        }

        #[pyfunction]
        pub fn calculate_pair_bbo(vals: Vec<PricePair>) -> PyResult<PricePair> {
            Ok(finlib::price::bbo::calculate_pair_bbo(vals))
        }
    }

    #[pymodule]
    mod risk {
        use super::*;

        #[pymodule]
        mod value_at_risk {
            use super::*;

            #[pyfunction]
            fn historical(values: Vec<f64>, confidence: f64) -> PyResult<f64> {
                Ok(finlib::risk::var::historical::value_at_risk_percent(
                    &values, confidence,
                ))
            }

            #[pyfunction]
            fn varcovar(values: Vec<f64>, confidence: f64) -> PyResult<f64> {
                Ok(finlib::risk::var::varcovar::value_at_risk_percent(
                    &values, confidence,
                ))
            }

            #[pymodule_export]
            use finlib::risk::var::scale_value_at_risk;
        }
    }

    #[pymodule]
    mod stats {
        use super::*;

        #[pyfunction]
        pub fn covariance(slice: Vec<f64>, slice_two: Vec<f64>) -> PyResult<Option<f64>> {
            Ok(finlib::stats::covariance(&slice, &slice_two))
        }
    }

    #[pymodule]
    mod util {
        use super::*;

        #[pyfunction]
        pub fn changes(slice: Vec<f64>) -> PyResult<Vec<f64>> {
            Ok(finlib::util::roc::changes(&slice).collect::<Vec<_>>())
        }

        #[pyfunction]
        pub fn rates_of_change(slice: Vec<f64>) -> PyResult<Vec<f64>> {
            Ok(finlib::util::roc::rates_of_change(&slice).collect::<Vec<_>>())
        }

        #[pyfunction]
        pub fn dot_product(a: Vec<f64>, b: Vec<f64>) -> PyResult<f64> {
            Ok(finlib::util::vector::dot_product(&a, &b))
        }

        #[pyfunction]
        pub fn mag(a: Vec<f64>) -> PyResult<f64> {
            Ok(finlib::util::vector::mag(&a))
        }
    }
}
