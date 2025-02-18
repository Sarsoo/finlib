use crate::util::roc::rates_of_change;
use rayon::prelude::*;

// https://www.simtrade.fr/blog_simtrade/historical-method-var-calculation/

pub fn value_at_risk(values: &[f64], confidence: f64) -> f64 {
    let mut roc = rates_of_change(values).collect::<Vec<_>>();

    roc.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let threshold = (confidence * roc.len() as f64).floor() as usize;

    roc[threshold]
}

pub fn par_value_at_risk(values: &[f64], confidence: f64) -> f64 {
    let mut roc = rates_of_change(values).collect::<Vec<_>>();

    roc.par_sort_by(|x, y| x.partial_cmp(y).unwrap());

    let threshold = (confidence * roc.len() as f64).floor() as usize;

    roc[threshold]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_test() {
        let result = value_at_risk(&[1f64, 2f64, 4f64, 5f64], 0.01f64);
        assert_eq!(result, 0.25f64);
    }
}