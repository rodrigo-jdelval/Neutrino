#![no_std]
pub const HEAP_BASE: usize = 0x50000;
pub const HEALTH_MATRIX: usize = 0x38400;
pub struct MemoryManager { pub current_ptr: usize }
impl MemoryManager {
    pub fn new() -> Self { unsafe { poke(0x38000, HEAP_BASE); } MemoryManager { current_ptr: HEAP_BASE } }
    pub fn alloc(&mut self, size: usize) -> usize {
        let mut offset = (self.current_ptr - HEAP_BASE) / 64;
        unsafe {
            while offset < 1024 {
                let h = peekb(HEALTH_MATRIX + offset);
                if h > 128 { break; }
                self.current_ptr += 64;
                offset += 1;
            }
        }
        let addr = self.current_ptr;
        self.current_ptr += (size + 3) & !3;
        unsafe { poke(0x38000, self.current_ptr); }
        addr
    }
    pub fn perform_scrubbing(&mut self) {}
}