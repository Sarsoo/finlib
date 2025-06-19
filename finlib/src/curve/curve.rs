use crate::curve::point::CurvePoint;
use crate::price::price::PricePair;
use chrono::NaiveDate;
#[cfg(feature = "py")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "btree_cursors")]
use std::ops::Bound;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum CurveType {
    Absolute,
    Differential,
}

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Serialize, Deserialize)]
pub struct Curve {
    tree: BTreeMap<NaiveDate, CurvePoint>,
    pub curve_type: CurveType,
}

impl Curve {
    pub fn new(curve_type: CurveType) -> Curve {
        Curve {
            tree: BTreeMap::new(),
            curve_type,
        }
    }

    pub fn from(curve_type: CurveType, values: impl IntoIterator<Item = CurvePoint>) -> Self {
        Curve {
            tree: BTreeMap::from_iter(values.into_iter().map(|x| (x.date, x))),
            curve_type,
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

    pub fn get_rate(&self, at: NaiveDate) -> Option<PricePair> {
        match self.curve_type {
            CurveType::Absolute => self.get_absolute_rate(at),
            CurveType::Differential => self.get_cumulative_rate(at),
        }
    }

    pub fn get_cumulative_rate(&self, at: NaiveDate) -> Option<PricePair> {
        if self.curve_type == CurveType::Absolute {
            return self.get_absolute_rate(at);
        }

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
                Err(_) => {
                    return None;
                }
            },
            (true, _, _) => return None,
            _ => {}
        }

        Some(cumulative)
    }

    pub fn get_absolute_rate(&self, at: NaiveDate) -> Option<PricePair> {
        let mut rate = PricePair::new();

        if let Some(direct_val) = self.tree.get(&at) {
            rate.bid = direct_val.bid_rate;
            rate.offer = direct_val.offer_rate;
        } else {
            let mut last_point_before_target: Option<&CurvePoint> = None;
            let mut first_point_after_target: Option<&CurvePoint> = None;

            #[cfg(feature = "btree_cursors")]
            {
                let cursor = self.tree.upper_bound(Bound::Excluded(&at));
                last_point_before_target = match cursor.peek_prev() {
                    Some(val) => Some(val.1),
                    _ => None,
                };
                first_point_after_target = match cursor.peek_next() {
                    Some(val) => Some(val.1),
                    _ => None,
                };
            }
            #[cfg(not(feature = "btree_cursors"))]
            {
                for (i, p) in self.tree.iter() {
                    if i < &at {
                        last_point_before_target = Some(p);
                    } else {
                        first_point_after_target = Some(p);
                        break;
                    }
                }
            }

            match (last_point_before_target, first_point_after_target) {
                (Some(lp), Some(fp)) => match Self::interpolate(lp, fp, at) {
                    Ok(p) => {
                        rate.offer = p.offer_rate;
                        rate.bid = p.bid_rate;
                    }
                    Err(_) => {
                        return None;
                    }
                },
                _ => {
                    return None;
                }
            }
        }

        Some(rate)
    }

    pub fn get_carry_rate(&self, from: NaiveDate, to: NaiveDate) -> Option<PricePair> {
        let from_rate = self.get_rate(from);
        let to_rate = self.get_rate(to);

        match (from_rate, to_rate) {
            (Some(from), Some(to)) => Some(PricePair {
                bid: to.bid - from.bid,
                offer: to.offer - from.offer,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_rate_retrieval() {
        let mut curve = Curve::new(CurveType::Differential);

        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 2).unwrap());
        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 3).unwrap());
        curve.add_rate_from(100., 101., NaiveDate::from_ymd_opt(2020, 1, 5).unwrap());

        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
                .unwrap()
                .bid,
            100.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
                .unwrap()
                .offer,
            101.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
                .unwrap()
                .bid,
            200.
        );
        assert_eq!(
            curve
                .get_rate(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
                .unwrap()
                .bid,
            200.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
                .unwrap()
                .offer,
            202.
        );

        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .unwrap()
                .bid,
            400.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .unwrap()
                .offer,
            404.
        );
    }

    #[test]
    fn test_ordering() {
        let mut curve = Curve::new(CurveType::Differential);

        curve.add_rate_from(3., 3., NaiveDate::from_ymd_opt(2020, 1, 4).unwrap());
        curve.add_rate_from(1., 1., NaiveDate::from_ymd_opt(2020, 1, 2).unwrap());
        curve.add_rate_from(2., 2., NaiveDate::from_ymd_opt(2020, 1, 3).unwrap());
        curve.add_rate_from(4., 4., NaiveDate::from_ymd_opt(2020, 1, 5).unwrap());

        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
                .unwrap()
                .bid,
            1.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 3).unwrap())
                .unwrap()
                .bid,
            3.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .unwrap()
                .bid,
            6.
        );
        assert_eq!(
            curve
                .get_cumulative_rate(NaiveDate::from_ymd_opt(2020, 1, 5).unwrap())
                .unwrap()
                .bid,
            10.
        );
    }

    #[test]
    fn test_absolute() {
        let mut curve = Curve::new(CurveType::Absolute);

        curve.add_rate_from(1., 1., NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        curve.add_rate_from(2., 2., NaiveDate::from_ymd_opt(2020, 1, 2).unwrap());
        curve.add_rate_from(3., 3., NaiveDate::from_ymd_opt(2020, 1, 3).unwrap());
        curve.add_rate_from(5., 5., NaiveDate::from_ymd_opt(2020, 1, 5).unwrap());

        assert_eq!(
            curve
                .get_absolute_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .unwrap()
                .bid,
            4.
        );

        assert_eq!(
            curve
                .get_rate(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap())
                .unwrap()
                .bid,
            4.
        );

        assert_eq!(
            curve
                .get_carry_rate(
                    NaiveDate::from_ymd_opt(2020, 1, 3).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 5).unwrap()
                )
                .unwrap()
                .bid,
            2.
        );
    }
}
