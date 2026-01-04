#![no_std]
pub struct Shard { pub id: u32, pub is_distributed: bool }
pub struct Fractalizer { pub count: u32 }
impl Fractalizer {
    pub fn new() -> Self { Fractalizer { count: 0 } }
    pub fn fractalize(&mut self, hash: u32) -> Shard {
        self.count += 1;
        Shard { id: self.count, is_distributed: hash > 0x8000 }
    }
}