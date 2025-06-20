use crate::price::enums::Side;

pub trait Payoff<T> {
    fn payoff(&self, underlying: T) -> f64;
}

pub trait Premium {
    fn premium(&self) -> f64;
    fn side(&self) -> Side;
    fn premium_payoff(&self) -> f64 {
        match self.side() {
            Side::Buy => -self.premium(),
            Side::Sell => self.premium(),
        }
    }
}

pub trait Profit<T>: Payoff<T> {
    fn profit(&self, underlying: T) -> f64;
}

#[macro_export]
macro_rules! impl_premium_profit {
    ($underlying:ty, $implemented_type:ty) => {
        impl Profit<$underlying> for $implemented_type {
            fn profit(&self, underlying: $underlying) -> f64 {
                self.payoff(underlying) + self.premium_payoff()
            }
        }
    };
}
