// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(debug_assertions)]
mod utils;
use js_sys::{Array, JsString, Reflect};
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn total(objects: &Array, key: &JsString) -> f64 {
    // use debug panic hook in dev
    #[cfg(debug_assertions)]
    utils::set_panic_hook();
    // convert array to vector, then iterator
    objects
        .to_vec()
        .iter()
        // use JS Reflect API get the value of an object key, if it fails, just return 0
        // Reflect will error if not called on an object, so we might handle this case
        .map(|p| match Reflect::get(&p, &key) {
            Ok(v) => v.as_f64().unwrap_or(0_f64),
            _ => 0_f64,
        })
        .fold(0_f64, |acc, i| acc + i)
}
