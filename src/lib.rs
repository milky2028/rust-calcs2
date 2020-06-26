#[cfg(target_arch = "wasm32")]
pub mod js_translation_layer;

#[cfg(target_arch = "x86_64")]
pub mod fns;
