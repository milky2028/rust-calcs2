//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
// use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use js_sys::{Array, JsString, JSON};
use wasm_bindgen_test::*;

#[path = "../src/lib.rs"]
mod lib;
use lib::*;

#[wasm_bindgen_test]
fn test_total() {
    let test_arr = Array::from(
        // create a test array by parsing JSON from a raw string
        &JSON::parse(r#"[{ "thing": 15 }, { "thing": 31 }, { "thing": 14 }]"#).unwrap(),
    );

    let res = total(&test_arr, &JsString::from("thing"));
    assert_eq!(60_f64, res)
}
