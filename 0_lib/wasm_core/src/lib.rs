#![no_std]
pub struct WasmInterpreter { pub pc: usize }
impl WasmInterpreter {
    pub fn new() -> Self { WasmInterpreter { pc: 0 } }
    pub fn step(&mut self, op: u8, arg: i32) { self.pc += 1; }
}