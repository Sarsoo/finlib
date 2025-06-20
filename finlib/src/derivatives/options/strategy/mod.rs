pub mod component;
pub mod strategy;

use crate::derivatives::options::OptionType;
use crate::price::payoff::{Payoff, Premium, Profit};
use std::sync::{Arc, Mutex};

pub trait IOptionStrategy: Payoff<f64> {
    fn components(&self) -> Vec<Arc<Mutex<dyn IOptionStrategyComponent>>>;
    fn add_component(&mut self, component: impl IOptionStrategyComponent + 'static);
    fn add_components(
        &mut self,
        components: impl IntoIterator<Item = impl IOptionStrategyComponent + 'static>,
    );
}

pub trait IOptionStrategyComponent: Payoff<f64> + Premium + Profit<f64> + Send {
    fn option_type(&self) -> OptionType;
    fn strike(&self) -> f64;
    fn will_be_exercised(&self, underlying: f64) -> bool;
}
