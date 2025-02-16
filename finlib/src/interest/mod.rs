//! Compound interest etc

pub fn compound_32(principal: f32, rate: f32, time: f32, n: f32) -> f32 {
    principal * f32::powf( 1f32 + (rate / n), time * n)
}

pub fn compound(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    principal * f64::powf( 1f64 + (rate / n), time * n)
}
/// https://www.thecalculatorsite.com/finance/calculators/compoundinterestcalculator.php

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_compound_32() {
        let result = compound_32(100f32, 0.05f32, 1f32, 1f32);
        assert_eq!(f32::round(result), 105f32);
    }

    #[test]
    fn monthly_compound_32() {
        let result = compound_32(100f32, 0.05f32, 1f32, 12f32);
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
