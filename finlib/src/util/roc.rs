
pub fn changes(values: &[f64]) -> impl Iterator<Item = f64> + use<'_> {
    values
        .windows(2)
        .map(|x| x[1] - x[0])
}

pub fn rates_of_change(values: &[f64]) -> impl Iterator<Item = f64> + use<'_> {
    values
        .windows(2)
        .map(|x| (x[1] - x[0])/x[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_test() {
        let result = changes(&[1f64, 2f64, 4f64, 5f64]).collect::<Vec<_>>();
        assert_eq!(result, vec![1f64, 2f64, 1f64]);
    }

    #[test]
    fn roc_test() {
        let result = rates_of_change(&[1f64, 2f64, 4f64, 5f64]).collect::<Vec<_>>();
        assert_eq!(result, vec![1f64, 1f64, 0.25f64]);
    }
}