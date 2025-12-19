use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Counter {
    value: u32,
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
    }

    #[wasm_bindgen]
    pub fn get(&self) -> u32 {
        self.value
    }
}
