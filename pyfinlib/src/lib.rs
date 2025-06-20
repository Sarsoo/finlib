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

            #[pyfunction]
            pub fn relative_strength_indicator(
                time_period: f64,
                average_gain: f64,
                average_loss: f64,
            ) -> PyResult<f64> {
                Ok(finlib::indicators::rsi::relative_strength_indicator(
                    time_period,
                    average_gain,
                    average_loss,
                ))
            }

            #[pyfunction]
            pub fn relative_strength_indicator_smoothed(
                time_period: f64,
                previous_average_gain: f64,
                current_gain: f64,
                previous_average_loss: f64,
                current_loss: f64,
            ) -> PyResult<f64> {
                Ok(
                    finlib::indicators::rsi::relative_strength_indicator_smoothed(
                        time_period,
                        previous_average_gain,
                        current_gain,
                        previous_average_loss,
                        current_loss,
                    ),
                )
            }
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
        use finlib::derivatives::options::blackscholes::CallOption;
        #[pymodule_export]
        use finlib::derivatives::options::blackscholes::OptionVariables;
        #[pymodule_export]
        use finlib::derivatives::options::blackscholes::PutOption;
        #[pymodule_export]
        use finlib::derivatives::options::strategy::component::OptionStrategyComponent;
        #[pymodule_export]
        use finlib::derivatives::options::strategy::strategy::OptionStrategy;
        #[pymodule_export]
        use finlib::derivatives::options::OptionGreeks;
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
                Ok(finlib::risk::var::historical::value_at_risk(
                    &values, confidence,
                ))
            }

            #[pyfunction]
            fn varcovar(values: Vec<f64>, confidence: f64) -> PyResult<f64> {
                Ok(finlib::risk::var::varcovar::value_at_risk_percent(
                    &values, confidence,
                ))
            }

            #[pyfunction]
            fn scale_value_at_risk(initial_value: f64, time_cycles: isize) -> PyResult<f64> {
                Ok(finlib::risk::var::varcovar::scale_value_at_risk(
                    initial_value,
                    time_cycles,
                ))
            }
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
