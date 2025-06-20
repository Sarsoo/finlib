use crate::derivatives::options::blackscholes::{CallOption, Option, OptionVariables, PutOption};
use ndarray::Array6;
use rayon::prelude::*;

pub fn generate_options(
    option_variables: Array6<OptionVariables>,
) -> Result<Array6<(CallOption, PutOption)>, ()> {
    let shape = option_variables.raw_dim();

    let vec = option_variables
        .into_iter()
        .map(|v| {
            let mut call = v.call();
            let mut put = v.put();

            call.calc_greeks();
            put.calc_greeks();

            (call, put)
        })
        .collect::<Vec<(CallOption, PutOption)>>();

    match Array6::<(CallOption, PutOption)>::from_shape_vec(shape, vec) {
        Ok(a) => Ok(a),
        Err(_) => Err(()),
    }
}

pub fn par_generate_options(
    option_variables: Array6<OptionVariables>,
) -> Result<Array6<(CallOption, PutOption)>, ()> {
    let shape = option_variables.raw_dim();

    let vec = option_variables
        .into_par_iter()
        .map(|v| {
            let mut call = v.call();
            let mut put = v.put();

            call.calc_greeks();
            put.calc_greeks();

            (call, put)
        })
        .collect::<Vec<(CallOption, PutOption)>>();

    match Array6::<(CallOption, PutOption)>::from_shape_vec(shape, vec) {
        Ok(a) => Ok(a),
        Err(_) => Err(()),
    }
}
