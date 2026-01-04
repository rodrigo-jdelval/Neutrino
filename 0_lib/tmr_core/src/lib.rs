#![no_std]
pub struct Triplet { pub a: u32, pub b: u32, pub c: u32, pub fault_count: u32 }
impl Triplet {
    pub fn new(v: u32) -> Self { Triplet { a: v, b: v, c: v, fault_count: 0 } }
    pub fn vote(&self) -> u32 { if self.a == self.b { self.a } else if self.a == self.c { self.a } else { self.b } }
    pub fn scrub(&mut self) -> bool {
        let truth = self.vote();
        let mut healed = false;
        if self.a != truth { self.a = truth; healed = true; }
        if self.b != truth { self.b = truth; healed = true; }
        if self.c != truth { self.c = truth; healed = true; }
        if healed { self.fault_count += 1; }
        healed
    }
}