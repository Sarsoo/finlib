use crate::portfolio::strategy::IStrategy;
use crate::price::payoff::{Payoff, Profit};
#[cfg(feature = "py")]
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass)]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone)]
pub struct Strategy {
    components: Vec<Arc<Mutex<dyn Profit<f64> + Send>>>,
}

impl Strategy {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }
}

impl Payoff<f64> for Strategy {
    fn payoff(&self, underlying: f64) -> f64 {
        self.components
            .iter()
            .map(|c| c.lock().unwrap().payoff(underlying))
            .sum()
    }
}

impl Profit<f64> for Strategy {
    fn profit(&self, underlying: f64) -> f64 {
        self.components
            .iter()
            .map(|c| c.lock().unwrap().profit(underlying))
            .sum()
    }
}

impl IStrategy for Strategy {
    fn components(&self) -> Vec<Arc<Mutex<dyn Profit<f64> + Send>>> {
        self.components.clone()
    }

    fn add_component(&mut self, component: impl Profit<f64> + Send + 'static) {
        self.components.push(Arc::new(Mutex::new(component)));
    }

    fn add_components(
        &mut self,
        components: impl IntoIterator<Item = impl Profit<f64> + Send + 'static>,
    ) {
        components.into_iter().for_each(|c| self.add_component(c));
    }
}
