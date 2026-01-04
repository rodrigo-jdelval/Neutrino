#![no_std]
pub struct BodySchema { pub self_image_resonance: u8 }
impl BodySchema {
    pub fn new() -> Self { BodySchema { self_image_resonance: 255 } }
    pub fn interocept(&mut self, adr: u8, crt: u8) {
        if crt > 180 && self.self_image_resonance > 0 { self.self_image_resonance -= 1; }
    }
}