use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Body {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

#[wasm_bindgen]
impl Body {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Body {
        Body {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        let gravity = 9.8;

        self.vy += gravity * dt;
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // jednoduchá "zem"
        if self.y > 300.0 {
            self.y = 300.0;
            self.vy *= -0.6; // odraz
        }
    }
}