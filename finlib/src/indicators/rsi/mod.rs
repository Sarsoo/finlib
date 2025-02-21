

pub fn relative_strength_indicator(time_period: f64, average_gain: f64, average_loss: f64) -> f64 {
    100. - (
        100. /
            (1. + (
                (average_gain / time_period)
                    /
                (average_loss / time_period)
                )
            )
    )
}

pub fn relative_strength_indicator_smoothed(time_period: f64, previous_average_gain: f64, current_gain: f64, previous_average_loss: f64, current_loss: f64) -> f64 {
    100. - (100. /
        (1. +
            (
                ((previous_average_gain * time_period) + current_gain)

                /

                ((previous_average_loss * time_period) + current_loss)
            )
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_strength_indicator_test() {
        let result = relative_strength_indicator(14., 1., 0.8);
        assert_eq!(f64::floor(result * 100.) / 100., 55.55);
    }
}
