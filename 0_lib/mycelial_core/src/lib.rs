#![no_std]
pub struct MycelialLink;
impl MycelialLink {
    pub fn new() -> Self { MycelialLink }
    pub fn hallucinate_sensor(&mut self, val: u32) -> u32 { if val == 0 { 22 } else { val } }
    pub fn update_trust(&mut self, res: u8) {}
}