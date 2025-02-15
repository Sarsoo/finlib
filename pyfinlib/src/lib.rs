use pyo3::prelude::*;

#[pymodule]
mod pyfinlib {
    use super::*;

    #[pymodule_export]
    use finlib::risk::portfolio::Portfolio;
    #[pymodule_export]
    use finlib::risk::portfolio::PortfolioAsset;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();
        Ok(())
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
    mod risk {
        use super::*;

        #[pymodule]
        mod var {
            use super::*;

            #[pyfunction]
            fn historical(values: Vec<f64>, confidence: f64) -> PyResult<f64> {
                Ok(finlib::risk::var::historical::value_at_risk(&values, confidence))
            }

            #[pyfunction]
            fn varcovar(values: Vec<f64>, confidence: f64) -> PyResult<f64> {
                Ok(finlib::risk::var::varcovar::value_at_risk_percent(&values, confidence))
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
        pub fn rates_of_change(slice: Vec<f64>) -> PyResult<Vec<f64>> {
            Ok(finlib::util::roc::rates_of_change(&slice).collect::<Vec<_>>())
        }
    }
}