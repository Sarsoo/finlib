use pyo3::prelude::*;

#[pyfunction]
pub fn compound(principal: f64, rate: f64, time: f64, n: f64) -> PyResult<f64> {
    Ok(finlib::interest::compound(principal, rate, time, n))
}

#[pyfunction]
pub fn covariance(slice: Vec<f64>, slice_two: Vec<f64>) -> PyResult<Option<f64>> {
    Ok(finlib::stats::covariance(&slice, &slice_two))
}

#[pymodule]
fn pyfinlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compound, m)?)?;
    m.add_function(wrap_pyfunction!(covariance, m)?)?;
    Ok(())
}