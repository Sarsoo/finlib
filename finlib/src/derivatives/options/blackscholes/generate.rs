use crate::derivatives::options::blackscholes::BlackscholesPricer;
use crate::derivatives::options::blackscholes::OptionVariables;
use crate::derivatives::options::{OptionContract, OptionStyle, OptionType};
use crate::price::enums::Side;
use ndarray::Array6;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use alloc::vec::Vec;

pub fn generate_options(
    option_variables: Array6<OptionVariables>,
) -> Result<Array6<(OptionContract, OptionContract)>, ()> {
    let shape = option_variables.raw_dim();

    let p = BlackscholesPricer {};
    let vec = option_variables
        .into_iter()
        .map(|v| process_var(&v, &p))
        .collect::<Vec<(OptionContract, OptionContract)>>();

    match Array6::<(OptionContract, OptionContract)>::from_shape_vec(shape, vec) {
        Ok(a) => Ok(a),
        Err(_) => Err(()),
    }
}

#[cfg(feature = "rayon")]
pub fn par_generate_options(
    option_variables: Array6<OptionVariables>,
) -> Result<Array6<(OptionContract, OptionContract)>, ()> {
    let shape = option_variables.raw_dim();

    let p = BlackscholesPricer {};
    let vec = option_variables
        .into_par_iter()
        .map(|v| process_var(v, &p))
        .collect::<Vec<(OptionContract, OptionContract)>>();

    match Array6::<(OptionContract, OptionContract)>::from_shape_vec(shape, vec) {
        Ok(a) => Ok(a),
        Err(_) => Err(()),
    }
}

fn process_var(v: &OptionVariables, p: &BlackscholesPricer) -> (OptionContract, OptionContract) {
    let mut call = OptionContract::from_vars(v, OptionType::Call, OptionStyle::European, Side::Buy);
    let mut put = OptionContract::from_vars(v, OptionType::Put, OptionStyle::European, Side::Buy);

    p.set_price(&mut call, &v);
    p.set_price(&mut put, &v);

    p.set_greeks(&mut call, &v);
    p.set_greeks(&mut put, &v);

    (call, put)
}
