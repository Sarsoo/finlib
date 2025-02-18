use rayon::prelude::*;
use crate::options::blackscholes::{CallOption, Option, OptionVariables, PutOption};

pub fn generate_options(option_variables: &Vec<OptionVariables>) -> Vec<(CallOption, PutOption)> {
    option_variables
        .iter()
        .map(|v| {
            let mut call = v.call();
            let mut put = v.put();

            call.calc_greeks();
            put.calc_greeks();

            (call, put)
        })
        .collect::<Vec<(CallOption, PutOption)>>()
}

pub fn par_generate_options(option_variables: &Vec<OptionVariables>) -> Vec<(CallOption, PutOption)> {
    option_variables
        .par_iter()
        .map(|v| {
            let mut call = v.call();
            let mut put = v.put();

            call.calc_greeks();
            put.calc_greeks();

            (call, put)
        })
        .collect::<Vec<(CallOption, PutOption)>>()
}