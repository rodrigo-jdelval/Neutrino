#![no_std]
pub type SerialDriver = VirtSerial;
pub type TimeDriver = VirtTime;
pub type BioDriver = VirtBio;
pub fn init() { Drivers::init(); }