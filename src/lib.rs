use js_sys::{Array, JsString, Reflect};
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn total(objects: &Array, key: &JsString) -> f64 {
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

// run tests with wasm-pack --node
#[cfg(test)]
mod tests {
    // looks into enclosing file/module for stuff
    use super::*;
    use js_sys::JSON;
    use wasm_bindgen_test::*;

    // test wrapper to run wasm tests in node. Does not work with usual cargo test
    #[wasm_bindgen_test]
    fn test_total() {
        let test_arr = Array::from(
            // create a test array by parsing JSON from a raw string
            &JSON::parse(r#"[{ "thing": 15 }, { "thing": 31 }, { "thing": 14 }]"#).unwrap(),
        );

        let res = total(&test_arr, &JsString::from("thing"));
        assert_eq!(60_f64, res)
    }
}
