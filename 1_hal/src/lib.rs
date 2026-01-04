#![no_std]
pub trait Serial { fn write_char(&self, c: u8); }
pub trait Time { fn now(&self) -> u32; }
pub trait Bio { fn set_hormone(&self, h: u8, v: u8); }