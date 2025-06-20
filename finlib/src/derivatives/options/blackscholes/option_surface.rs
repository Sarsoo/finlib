use crate::derivatives::options::blackscholes::{
    generate_options, par_generate_options, CallOption, OptionVariables, PutOption,
};
use core::ops::Range;
use ndarray::Array6;

#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptionSurfaceParameters {
    underlying_price: Range<isize>,
    underlying_price_bounds: (f64, f64),
    strike_price: Range<isize>,
    strike_price_bounds: (f64, f64),
    volatility: Range<isize>,
    volatility_bounds: (f64, f64),
    risk_free_interest_rate: Range<isize>,
    risk_free_interest_rate_bounds: (f64, f64),
    dividend: Range<isize>,
    dividend_bounds: (f64, f64),
    time_to_expiration: Range<isize>,
    time_to_expiration_bounds: (f64, f64),
}

impl OptionSurfaceParameters {
    pub fn from(
        underlying_price: Range<isize>,
        underlying_price_bounds: (f64, f64),
        strike_price: Range<isize>,
        strike_price_bounds: (f64, f64),
        volatility: Range<isize>,
        volatility_bounds: (f64, f64),
        risk_free_interest_rate: Range<isize>,
        risk_free_interest_rate_bounds: (f64, f64),
        dividend: Range<isize>,
        dividend_bounds: (f64, f64),
        time_to_expiration: Range<isize>,
        time_to_expiration_bounds: (f64, f64),
    ) -> Self {
        Self {
            underlying_price,
            underlying_price_bounds,
            strike_price,
            strike_price_bounds,
            volatility,
            volatility_bounds,
            risk_free_interest_rate,
            risk_free_interest_rate_bounds,
            dividend,
            dividend_bounds,
            time_to_expiration,
            time_to_expiration_bounds,
        }
    }

    fn scale(bound: (f64, f64), index: isize, length: usize) -> f64 {
        bound.0 + (bound.1 - bound.0) * (index as f64 / length as f64)
    }

    pub fn walk(self) -> Result<OptionsSurface, ()> {
        let mut vec: Vec<OptionVariables> = Vec::with_capacity(
            self.underlying_price.len()
                * self.strike_price.len()
                * self.volatility.len()
                * self.risk_free_interest_rate.len()
                * self.dividend.len()
                * self.time_to_expiration.len(),
        );
        let shape = [
            self.underlying_price.len(),
            self.strike_price.len(),
            self.volatility.len(),
            self.risk_free_interest_rate.len(),
            self.dividend.len(),
            self.time_to_expiration.len(),
        ];
        for p in self.underlying_price.clone() {
            for s in self.strike_price.clone() {
                for v in self.volatility.clone() {
                    for i in self.risk_free_interest_rate.clone() {
                        for d in self.dividend.clone() {
                            for t in self.time_to_expiration.clone() {
                                let v = OptionVariables::from(
                                    Self::scale(
                                        self.underlying_price_bounds,
                                        p,
                                        self.underlying_price.len(),
                                    ),
                                    Self::scale(
                                        self.strike_price_bounds,
                                        s,
                                        self.strike_price.len(),
                                    ),
                                    Self::scale(self.volatility_bounds, v, self.volatility.len()),
                                    Self::scale(
                                        self.risk_free_interest_rate_bounds,
                                        i,
                                        self.risk_free_interest_rate.len(),
                                    ),
                                    Self::scale(self.dividend_bounds, d, self.dividend.len()),
                                    Self::scale(
                                        self.time_to_expiration_bounds,
                                        t,
                                        self.time_to_expiration.len(),
                                    ),
                                );
                                vec.push(v);
                            }
                        }
                    }
                }
            }
        }

        match Array6::<OptionVariables>::from_shape_vec(shape, vec) {
            Ok(a) => Ok(OptionsSurface::from(a)),
            Err(_) => Err(()),
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptionsSurface {
    variables: Option<Array6<OptionVariables>>,
    options: Option<Array6<(CallOption, PutOption)>>,
}

impl OptionsSurface {
    pub fn from(variables: Array6<OptionVariables>) -> Self {
        Self {
            variables: Some(variables),
            options: None,
        }
    }

    pub fn len(&self) -> usize {
        match (self.variables.is_some(), self.options.is_some()) {
            (true, false) => self.variables.as_ref().unwrap().len(),
            (false, true) => self.options.as_ref().unwrap().len(),
            (true, true) => 0,
            (false, false) => 0,
        }
    }

    pub fn generate(&mut self) -> Result<(), ()> {
        let variables = self.variables.take();
        match variables {
            Some(v) => match generate_options(v) {
                Ok(o) => {
                    self.options = Some(o);
                    Ok(())
                }
                Err(_) => Err(()),
            },
            None => Err(()),
        }
    }

    pub fn par_generate(&mut self) -> Result<(), ()> {
        let variables = self.variables.take();
        match variables {
            Some(v) => match par_generate_options(v) {
                Ok(o) => {
                    self.options = Some(o);
                    Ok(())
                }
                Err(_) => Err(()),
            },
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_test() {
        let w = OptionSurfaceParameters::from(
            0..50,
            (100., 200.),
            0..50,
            (100., 200.),
            0..5,
            (0.25, 0.50),
            0..10,
            (0.05, 0.08),
            0..1,
            (0.01, 0.02),
            0..10,
            (30. / 365.25, 30. / 365.25),
        );

        let mut a = w.walk().unwrap();
        a.generate();

        let _ = a.options.unwrap()[[0, 0, 0, 0, 0, 0]];
    }
}
