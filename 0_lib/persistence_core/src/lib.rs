#![no_std]
pub struct Lazarus;
impl Lazarus {
    pub fn snapshot(a: u8, c: u8, m: u8, i: u8) {}
    pub fn can_resurrect() -> bool { false }
    pub fn get_adr() -> u8 { 0 }
    pub fn get_crt() -> u8 { 0 }
    pub fn get_mlt() -> u8 { 0 }
    pub fn get_integrity() -> u8 { 255 }
}