#[path = "./fns.rs"]
mod fns;
use fns::{total, Data};

use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn total_value(items: JsValue) -> f64 {
    match from_value::<Vec<Data>>(items) {
        Ok(v) => total(&v),
        _ => 0_f64,
    }
}
