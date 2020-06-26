#[cfg(target_arch = "wasm32")]
pub mod js_translation_layer;

#[cfg(not(target_arch = "wasm32"))]
pub mod fns;
