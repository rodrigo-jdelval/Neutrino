#![no_std]
pub struct Mailbox { pub msg: u32, pub locked: bool }
impl Mailbox {
    pub fn new() -> Self { Mailbox { msg: 0, locked: false } }
    pub fn send(&mut self, d: u32) -> bool { if self.locked { false } else { self.msg=d; self.locked=true; true } }
    pub fn recv(&mut self) -> u32 { if !self.locked { 0 } else { self.locked=false; self.msg } }
}