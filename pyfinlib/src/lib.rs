use pyo3::prelude::*;

#[pyfunction]
fn add(left: u64, right: u64) -> PyResult<u64> {
    Ok(finlib::add(left, right))
}

#[pyfunction]
pub fn compound(principal: f32, rate: f32, time: f32, n: f32) -> PyResult<f32> {
    Ok(finlib::interest::compound(principal, rate, time, n))
}

#[pymodule]
fn pyfinlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(compound, m)?)?;
    Ok(())
}