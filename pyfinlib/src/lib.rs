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
    register_interest_module(m);
    register_stats_module(m);
    Ok(())
}

fn register_interest_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "interest")?;
    child_module.add_function(wrap_pyfunction!(compound, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}

fn register_stats_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "stats")?;
    child_module.add_function(wrap_pyfunction!(covariance, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}