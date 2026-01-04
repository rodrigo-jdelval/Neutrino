#![no_std]
pub struct SbcEncoder;
impl SbcEncoder {
    pub fn pack(adr: u8, crt: u8, mlt: u8) -> u16 {
        ((adr as u16 >> 3) << 11) | ((crt as u16 >> 3) << 6) | (mlt as u16 >> 2)
    }
}