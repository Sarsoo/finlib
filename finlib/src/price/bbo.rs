use crate::price::enums::Side;
use crate::price::price::{Price, PricePair};

pub fn calculate_bbo<I>(vals: I) -> PricePair
where I: IntoIterator<Item = Price>,
{
    let mut highest_buy: f64 = 0.;
    let mut lowest_sell = f64::MAX;
    for i in vals {
        match i.side {
            Side::Buy => {
                if i.value > highest_buy {
                    highest_buy = i.value;
                }
            }
            Side::Sell => {
                if i.value < lowest_sell {
                    lowest_sell = i.value;
                }
            }
        }
    }

    PricePair {
        bid: highest_buy, offer: lowest_sell
    }
}

pub fn calculate_pair_bbo<T>(vals: T) -> PricePair
where T: IntoIterator<Item = PricePair>,
{
    let mut highest_buy = 0.;
    let mut lowest_sell = f64::MAX;
    for i in vals {
        if i.bid > highest_buy {
            highest_buy = i.bid;
        }
        if i.offer < lowest_sell {
            lowest_sell = i.offer;
        }
    }

    PricePair {
        bid: highest_buy, offer: lowest_sell
    }
}