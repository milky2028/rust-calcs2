//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
// use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use js_sys::JSON;
use wasm_bindgen_test::*;

#[path = "../src/js_translation_layer.rs"]
mod web_layer;
use web_layer::total_value;

#[wasm_bindgen_test]
fn test_total() {
    let test_arr = JSON::parse(
        r#"[{ "id": "first", "value": 30 }, { "id": "first", "value": 30 }, { "id": "first", "value": 30 }]"#,
    )
    .unwrap();

    let res = total_value(test_arr);
    assert_eq!(90_f64, res)
}
