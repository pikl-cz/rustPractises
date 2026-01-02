use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;

#[wasm_bindgen]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter { value: 0 }
    }

    // Pozor: musí být pub a &mut self
    #[wasm_bindgen]
    pub fn increment(&mut self) {
        self.value += 1;
        console::log_1(&JsValue::from(self.value));
    }

    #[wasm_bindgen]
    pub fn get(&self) -> i32 {
        self.value
    }
}
