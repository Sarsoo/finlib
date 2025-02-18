use super::mean;

pub fn covariance(slice: &[f64], slice_two: &[f64]) -> Option<f64>
{
    match slice.len() - slice_two.len() {
        0 => {
            let mean_1 = mean(slice);
            let mean_2 = mean(slice_two);

            Some(slice
                .iter()
                .zip(slice_two
                    .iter()
                )
                .map(|(x, y)| (x - mean_1) * (y - mean_2))
                .sum::<f64>()
            / ((slice.len() - 1) as f64)
            )
        }
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covariance_test() {
        let result = covariance(&[1f64, 2f64, 3f64, 4f64], &[1f64, 2f64, 3f64, 4f64]);
        assert_eq!(result.unwrap(), 1.6666666666666667f64);
    }

    #[test]
    fn covariance_test_break() {
        let result = covariance(&[1f64, 2f64, 3f64, 4f64], &[1f64, 2f64]);
        assert_eq!(result.is_none(), true);
    }
}