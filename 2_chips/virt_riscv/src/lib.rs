#![no_std]
pub struct VirtSerial;
impl VirtSerial { pub fn new() -> Self { VirtSerial } pub fn write_char(&self, c: u8) { unsafe { poke(0x60000, c as usize); } } }
pub struct VirtTime;
impl VirtTime { pub fn new() -> Self { VirtTime } pub fn now(&self) -> u32 { unsafe { peek(0x38304) as u32 } } }
pub struct VirtBio;
impl VirtBio { pub fn new() -> Self { VirtBio } pub fn set_hormone(&self, h: u8, v: u8) { unsafe { poke(0x38200 + h as usize, v as usize); } } }