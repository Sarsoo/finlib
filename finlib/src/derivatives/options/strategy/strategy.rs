use crate::derivatives::options::strategy::{IOptionStrategy, IOptionStrategyComponent};
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
pub struct OptionStrategy {
    components: Vec<Arc<Mutex<dyn IOptionStrategyComponent>>>,
}

impl OptionStrategy {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }
}

impl Payoff<f64> for OptionStrategy {
    fn payoff(&self, underlying: f64) -> f64 {
        self.components
            .iter()
            .map(|c| c.lock().unwrap().payoff(underlying))
            .sum()
    }
}

impl Profit<f64> for OptionStrategy {
    fn profit(&self, underlying: f64) -> f64 {
        self.components
            .iter()
            .map(|c| c.lock().unwrap().profit(underlying))
            .sum()
    }
}

impl IOptionStrategy for OptionStrategy {
    fn components(&self) -> Vec<Arc<Mutex<dyn IOptionStrategyComponent>>> {
        self.components.clone()
    }

    fn add_component(&mut self, component: impl IOptionStrategyComponent + 'static) {
        self.components.push(Arc::new(Mutex::new(component)));
    }

    fn add_components(
        &mut self,
        components: impl IntoIterator<Item = impl IOptionStrategyComponent + 'static>,
    ) {
        components.into_iter().for_each(|c| self.add_component(c));
    }
}
