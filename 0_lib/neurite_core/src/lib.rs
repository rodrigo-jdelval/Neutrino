#![no_std]
pub struct HormonalBus { pub adrenaline: u8, pub cortisol: u8, pub melatonin: u8 }
impl HormonalBus {
    pub fn new() -> Self { HormonalBus { adrenaline: 0, cortisol: 0, melatonin: 0 } }
    pub fn update(&mut self, stress: u8) {
        if self.adrenaline > 0 { self.adrenaline -= 1; }
        if stress > 0 { self.adrenaline = 255; self.melatonin = 0; }
        else if self.melatonin < 200 { self.melatonin += 1; }
    }
    pub fn get_adr(&self) -> u8 { self.adrenaline }
    pub fn get_crt(&self) -> u8 { self.cortisol }
    pub fn get_mlt(&self) -> u8 { self.melatonin }
}