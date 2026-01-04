#![no_std]
pub struct ImmuneSystem { pub resistance: u8, pub tar_pit_active: bool, pub antigen_count: u32 }
impl ImmuneSystem {
    pub fn new() -> Self { ImmuneSystem { resistance: 255, tar_pit_active: false, antigen_count: 0 } }
    pub fn scan(&mut self, alignment: u8) { 
        if alignment < 128 { 
            self.tar_pit_active = true;
            self.antigen_count += 1;
            if self.resistance > 0 { self.resistance -= 1; } 
        } else {
            self.tar_pit_active = false;
        }
    }
    pub fn should_jitter(&self, rnd: u32) -> bool {
        if !self.tar_pit_active { return false; }
        (rnd % 255) < (255 - self.resistance) as u32
    }
}