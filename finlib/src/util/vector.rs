use log::error;

pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        error!("Can't dot product two vectors of different lengths, a = {}, b = {}", a.len(), b.len());
    }
    assert_eq!(a.len(), b.len());

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}

pub fn mag(a: &[f64]) -> f64 {
    f64::sqrt(
        a.iter()
            .map(|x| f64::powi(*x, 2))
            .sum()
    )
}