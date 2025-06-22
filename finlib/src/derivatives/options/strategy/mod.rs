pub mod strategy;
pub mod templates;

use crate::derivatives::options::IOption;
use crate::price::payoff::Payoff;
use std::sync::{Arc, Mutex};

pub trait IOptionStrategy: Payoff<f64> {
    fn components(&self) -> Vec<Arc<Mutex<dyn IOption>>>;
    fn add_component(&mut self, component: impl IOption + 'static);
    fn add_components(&mut self, components: impl IntoIterator<Item = impl IOption + 'static>);
}
