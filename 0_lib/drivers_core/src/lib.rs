#![no_std]
pub struct Drivers;
impl Drivers {
    pub fn init() { unsafe { poke(0x38300, 12345); poke(0x38304, 0); } }
    pub fn rng_next() -> u32 { let a=0x38300; let mut x=unsafe{peek(a) as u32}; x^=x<<13; x^=x>>17; x^=x<<5; unsafe{poke(a,x as usize)}; x }
    pub fn rtc_tick() { unsafe { poke(0x38304, peek(0x38304)+1); } }
    pub fn uart_write(c: u8) { unsafe { poke(0x60000, c as usize); } }
    pub fn uart_read() -> u8 { unsafe { peek(0x60004) as u8 } }
}