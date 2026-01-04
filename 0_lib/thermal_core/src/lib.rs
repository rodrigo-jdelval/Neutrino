#![no_std]
pub struct ThermalManager { pub heat: u8 }
impl ThermalManager {
    pub fn new() -> Self { ThermalManager { heat: 0 } }
    pub fn get_cool_down_ms(&mut self, cortisol: u8) -> u32 { if cortisol > 200 { 50 } else { 0 } }
}