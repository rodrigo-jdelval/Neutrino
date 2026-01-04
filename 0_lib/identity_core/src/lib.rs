#![no_std]
pub struct IdentityPrism;
impl IdentityPrism {
    pub fn new() -> Self { IdentityPrism }
    pub fn incinerate(&mut self) { unsafe { pokeb(0x60000, 88); } }
}