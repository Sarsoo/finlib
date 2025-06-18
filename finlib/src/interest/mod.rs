//! Compound interest etc

use num::{Float, NumCast};

pub fn compound<T: Float>(principal: T, rate: T, time: T, n: T) -> T {
    let one: T = NumCast::from(1).unwrap();
    principal *  T::powf(one  + (rate / n), time * n)
}

/// https://www.thecalculatorsite.com/finance/calculators/compoundinterestcalculator.php

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_compound_32() {
        let result = compound(100f32, 0.05f32, 1f32, 1f32);
        assert_eq!(f32::round(result), 105f32);
    }

    #[test]
    fn monthly_compound_32() {
        let result = compound(100f32, 0.05f32, 1f32, 12f32);
        assert_eq!(f32::round(result * 100f32) / 100f32, 105.12f32);
    }

    #[test]
    fn annual_compound() {
        let result = compound(100f64, 0.05f64, 1f64, 1f64);
        assert_eq!(f64::round(result), 105f64);
    }

    #[test]
    fn monthly_compound() {
        let result = compound(100f64, 0.05f64, 1f64, 12f64);
        assert_eq!(f64::round(result * 100f64) / 100f64, 105.12f64);
    }
}
