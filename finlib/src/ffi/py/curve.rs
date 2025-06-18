use pyo3::prelude::*;
use crate::curve::curve::Curve;

#[pymethods]
impl Curve {

    #[new]
    pub fn init() -> Self {
        Self::new()
    }

    #[pyo3(name = "size")]
    pub fn size_py(&mut self) -> PyResult<usize> {
        Ok(self.size())
    }
}

