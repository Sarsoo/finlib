
pub fn compound(principal: f32, rate: f32, time: f32, n: f32) -> f32 {
    principal * f32::powf( 1f32 + (rate / n), time * n)
}
/// https://www.thecalculatorsite.com/finance/calculators/compoundinterestcalculator.php

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_compound() {
        let result = compound(100f32, 0.05f32, 1f32, 1f32);
        assert_eq!(f32::round(result), 105f32);
    }

    #[test]
    fn monthly_compound() {
        let result = compound(100f32, 0.05f32, 1f32, 12f32);
        assert_eq!(f32::round(result * 100f32) / 100f32, 105.12f32);
    }
}
