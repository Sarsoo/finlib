//! Compound interest etc

use num::{Float, NumCast};

/// Compound the provided `principal` by `rate` (0.05 or 5%) where `rate` is compounded for `time` periods where each `time` period has `n` interest payments
///
/// e.g a `principal` of 10,000 with an annual rate of 5% (`rate` = 0.05) for 10 years (`time` = 10) with monthly payments (`n` = 12)
pub fn compounded_principal<T: Float>(principal: T, rate: T, time: T, n: T) -> T {
    principal * compound_rate_per_n(rate, time, n)
}

/// Compound the provided `rate` (0.05 or 5%) into a compounded rate for `time` periods where each `time` period has `n` interest payments
///
/// e.g an annual rate of 5% (`rate` = 0.05) for 10 years (`time` = 10) with monthly payments (`n` = 12)
pub fn compound_rate_per_n<T: Float>(rate: T, time: T, n: T) -> T {
    rate_per_n(rate, n).powf(time * n)
}

pub fn anticompound_rate_per_n<T: Float>(rate: T, time: T, n: T) -> T {
    rate_per_n(rate, n).powf(-time * n)
}

/// Turn a `rate` = 0.05 (5%) into a scaling rate (1.05 or 105%) over `n` periods (`n` = 12 for an interest payment every month given an annual interest rate)
pub fn rate_per_n<T: Float>(rate: T, n: T) -> T {
    let one: T = NumCast::from(1).unwrap();
    one + (rate / n)
}

/// https://www.thecalculatorsite.com/finance/calculators/compoundinterestcalculator.php

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compound_uncompound() {
        let val = 10000.;

        let scaled = val * compound_rate_per_n(0.05, 10., 12.);

        let unscaled = scaled * anticompound_rate_per_n(0.05, 10., 12.);

        assert_eq!(val, unscaled);
    }

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
