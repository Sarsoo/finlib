pub mod strategy;

pub use strategy::*;

use crate::price::payoff::{Payoff, Profit};
use alloc::sync::Arc;
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use spin::mutex::Mutex;
#[cfg(feature = "std")]
use std::sync::Mutex;

pub trait IStrategy: Payoff<f64> {
    fn components(&self) -> Vec<Arc<Mutex<dyn Profit<f64> + Send>>>;
    fn add_component(&mut self, component: impl Profit<f64> + Send + 'static);
    fn add_components(
        &mut self,
        components: impl IntoIterator<Item = impl Profit<f64> + Send + 'static>,
    );
}
