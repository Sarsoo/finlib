use crate::derivatives::TradeSide;
use crate::price::enums::Side;

pub trait Payoff<T> {
    fn payoff(&self, underlying: T) -> f64;
}

pub trait Premium: TradeSide {
    fn premium(&self) -> f64;
    fn premium_payoff(&self) -> f64 {
        match self.side() {
            Side::Buy => -self.premium(),
            Side::Sell => self.premium(),
        }
    }
}

#[macro_export]
macro_rules! impl_premium {
    ($implemented_type:ty) => {
        impl Premium for $implemented_type {
            fn premium(&self) -> f64 {
                self.premium
            }
        }
    };
    ($implemented_type:ty:$parameter:ident) => {
        impl Premium for $implemented_type {
            fn premium(&self) -> f64 {
                self.$parameter
            }
        }
    };
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
