use crate::curve::point::CurvePoint;
use crate::price::price::PricePair;
use chrono::NaiveDate;
#[cfg(feature = "py")]
use pyo3::prelude::*;
use std::collections::BTreeMap;
// #[cfg(feature = "wasm")]
// use wasm_bindgen::prelude::*;

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Curve {
    tree: BTreeMap<NaiveDate, CurvePoint>,
}

impl Curve {
    pub fn new() -> Curve {
        Curve {
            tree: BTreeMap::new(),
        }
    }

    pub fn from(values: impl IntoIterator<Item = CurvePoint>) -> Self {
        Curve {
            tree: BTreeMap::from_iter(values.into_iter().map(|x| (x.date, x))),
        }
    }

    pub fn size(&self) -> usize {
        self.tree.len()
    }

    pub fn add_rate(&mut self, point: CurvePoint) {
        self.tree.insert(point.date, point);
    }

    pub fn add_rate_from(&mut self, bid: f64, offer: f64, date: NaiveDate) {
        self.tree.insert(
            date,
            CurvePoint {
                date,
                bid_rate: bid,
                offer_rate: offer,
            },
        );
    }

    pub fn interpolate(
        from: &CurvePoint,
        to: &CurvePoint,
        date: NaiveDate,
    ) -> Result<CurvePoint, ()> {
        if to.date < from.date || date < from.date || date > to.date {
            return Err(());
        }

        let width = to.date - from.date;
        let target_width = date - from.date;

        if width.num_days() == 0 {
            // return Ok(from.clone());
            return Err(());
        }
        if target_width.num_days() == 0 {
            // return Ok(from.clone());
            return Err(());
        }

        let bid_delta = to.bid_rate - from.bid_rate;
        let offer_delta = to.offer_rate - from.offer_rate;

        let date_weight = (target_width.num_days() as f64) / (width.num_days() as f64);

        Ok(CurvePoint {
            bid_rate: (bid_delta * date_weight) + from.bid_rate,
            offer_rate: (offer_delta * date_weight) + from.offer_rate,
            date,
        })
    }

    pub fn get_cumulative_rate(&self, at: NaiveDate) -> PricePair {
        let mut cumulative = PricePair::new();
        let mut last_point_before_target: Option<&CurvePoint> = None;
        let mut first_point_after_target: Option<&CurvePoint> = None;
        let mut interpolation_required = true;
        for (i, p) in self.tree.iter() {
            if i < &at {
                cumulative.bid += p.bid_rate;
                cumulative.offer += p.offer_rate;
                last_point_before_target = Some(p);
            } else if i == &at {
                interpolation_required = false;
                cumulative.bid += p.bid_rate;
                cumulative.offer += p.offer_rate;
                break;
            } else {
                first_point_after_target = Some(p);
                break;
            }
        }

        match (
            interpolation_required,
            last_point_before_target,
            first_point_after_target,
        ) {
            (true, Some(lp), Some(fp)) => match Self::interpolate(lp, fp, at) {
                Ok(p) => {
                    cumulative.offer += p.offer_rate;
                    cumulative.bid += p.bid_rate;
                }
                Err(_) => {}
            },
            _ => {}
        }

        cumulative
    }

    pub fn get_carry_rate(&self, from: NaiveDate, to: NaiveDate) -> PricePair {
        let from_rate = self.get_cumulative_rate(from);
        let to_rate = self.get_cumulative_rate(to);

        PricePair {
            bid: to_rate.bid - from_rate.bid,
            offer: to_rate.offer - from_rate.offer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_rate_retrieval() {
        let mut curve = Curve::new();

        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 2).unwrap());
        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 3).unwrap());
        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 5).unwrap());

        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
                .bid,
            100.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
                .offer,
            101.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
                .bid,
            200.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
                .offer,
            202.
        );

        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .bid,
            200.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .offer,
            202.
        );
    }

    #[test]
    fn var_test() {
        let mut curve = Curve::new();

        curve.add_rate_from(3., 3., NaiveDate::from_ymd_opt(2020, 1, 4).unwrap());
        curve.add_rate_from(1., 1., NaiveDate::from_ymd_opt(2020, 1, 2).unwrap());
        curve.add_rate_from(2., 2., NaiveDate::from_ymd_opt(2020, 1, 3).unwrap());
        curve.add_rate_from(4., 4., NaiveDate::from_ymd_opt(2020, 1, 5).unwrap());

        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
                .bid,
            1.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 3).unwrap())
                .bid,
            3.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .bid,
            6.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 5).unwrap())
                .bid,
            10.
        );
    }
}
