use crate::price::enums::Side;

pub mod options;
pub mod swaps;

pub trait TradeSide {
    fn side(&self) -> Side;
}

#[macro_export]
macro_rules! impl_side {
    ($implemented_type:ty) => {
        impl TradeSide for $implemented_type {
            fn side(&self) -> Side {
                self.side
            }
        }
    };
    ($implemented_type:ty:$parameter:ident) => {
        impl TradeSide for $implemented_type {
            fn side(&self) -> Side {
                self.$parameter
            }
        }
    };
}
