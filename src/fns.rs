#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "x86_64", repr(C))]
#[cfg_attr(target_arch = "wasm32", derive(Serialize, Deserialize))]
pub struct Data {
    pub id: String,
    pub value: f64,
}

#[no_mangle]
pub fn total(data: &Vec<Data>) -> f64 {
    data.iter().fold(0_f64, |acc, i| acc + i.value)
}
