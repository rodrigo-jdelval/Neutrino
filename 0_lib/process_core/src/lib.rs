#![no_std]
pub const MAX_PROCS: usize = 4;
pub struct Process { pub pid: u32, pub state: u32, pub vm: WasmInterpreter }
pub struct ProcessManager { pub procs: [Process; 4] }
impl ProcessManager {
    pub fn new() -> Self { 
        // Dummy Init
        let vm = WasmInterpreter::new();
        ProcessManager { procs: [Process { pid: 0, state: 0, vm: vm }; 4] } 
    }
    pub fn spawn(&mut self) -> u32 { 1 }
    pub fn schedule(&mut self) -> usize { 0 }
}