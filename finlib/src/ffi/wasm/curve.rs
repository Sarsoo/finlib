use crate::curve::curve::{Curve, CurveType};
use crate::price::price::PricePair;
use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Curve {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(curve_type: CurveType) -> Self {
        Self::new(curve_type)
    }

    #[wasm_bindgen(getter = length)]
    pub fn len_wasm(&self) -> usize {
        self.size()
    }

    #[wasm_bindgen(js_name = "addRateFrom")]
    pub fn add_rate_from_wasm(&mut self, bid: f64, offer: f64, date: js_sys::Date) {
        let date = DateTime::<Utc>::from(&date);
        self.add_rate_from(bid, offer, date.date_naive());
    }

    #[wasm_bindgen(js_name = "getCumulativeRate")]
    pub fn get_cumulative_rate_wasm(&mut self, at: js_sys::Date) -> Option<PricePair> {
        let at = DateTime::<Utc>::from(&at);
        self.get_cumulative_rate(at.date_naive())
    }

    #[wasm_bindgen(js_name = "getAbsoluteRate")]
    pub fn get_absolute_rate_wasm(&mut self, at: js_sys::Date) -> Option<PricePair> {
        let at = DateTime::<Utc>::from(&at);
        self.get_absolute_rate(at.date_naive())
    }

    #[wasm_bindgen(js_name = "getRate")]
    pub fn get_rate_wasm(&mut self, at: js_sys::Date) -> Option<PricePair> {
        let at = DateTime::<Utc>::from(&at);
        self.get_rate(at.date_naive())
    }

    #[wasm_bindgen(js_name = "getCarryRate")]
    pub fn get_carry_rate_wasm(
        &mut self,
        from: js_sys::Date,
        to: js_sys::Date,
    ) -> Option<PricePair> {
        let from = DateTime::<Utc>::from(&from);
        let to = DateTime::<Utc>::from(&to);
        self.get_carry_rate(from.date_naive(), to.date_naive())
    }
}
