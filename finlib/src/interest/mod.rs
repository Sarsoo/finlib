//! Compound interest etc

use num::traits::{Inv, MulAdd};
use num::{Float, Num, NumCast, ToPrimitive};
use std::ops::Neg;

pub fn compounded_principal<T: Float>(principal: T, rate: T, time: T, n: T) -> T {
    principal * compound_rate_per_n(rate, time, n)
}

pub fn compound_rate_per_n<T: Float>(rate: T, time: T, n: T) -> T {
    rate_per_n(rate, n).powf(time * n)
}

pub fn compound_rate_per_n_neg<T: Float>(rate: T, time: T, n: T) -> T {
    rate_per_n(rate, n).powf(-time * n)
}

pub fn rate_per_n<T: Float>(rate: T, n: T) -> T {
    let one: T = NumCast::from(1).unwrap();
    one + (rate / n)
}

/// https://www.thecalculatorsite.com/finance/calculators/compoundinterestcalculator.php

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_compound_32() {
        let result = compounded_principal(100f32, 0.05f32, 1f32, 1f32);
        assert_eq!(f32::round(result), 105f32);
    }

    #[test]
    fn monthly_compound_32() {
        let result = compounded_principal(100f32, 0.05f32, 1f32, 12f32);
        assert_eq!(f32::round(result * 100f32) / 100f32, 105.12f32);
    }

    #[test]
    fn annual_compound() {
        let result = compounded_principal(100f64, 0.05f64, 1f64, 1f64);
        assert_eq!(f64::round(result), 105f64);
    }

    #[test]
    fn monthly_compound() {
        let result = compounded_principal(100f64, 0.05f64, 1f64, 12f64);
        assert_eq!(f64::round(result * 100f64) / 100f64, 105.12f64);
    }
}
