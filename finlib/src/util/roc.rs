use num::{Float};

pub fn changes<T: Float>(values: &[T]) -> impl Iterator<Item = T> + use<'_, T> {
    values
        .windows(2)
        .map(|x| x[1] - x[0])
}

pub fn rates_of_change<T: Float>(values: &[T]) -> impl Iterator<Item = T> + use<'_, T> {
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