// libaether: THE ORGANISM LIBRARY v1.1
pub mod bio {
    pub fn panic_button() { unsafe { syscall(200, 0, 255, 0, 0); } }
    pub fn meditate() { unsafe { syscall(200, 2, 255, 0, 0); } }
}
pub mod fs {
    pub fn open(p: usize) -> usize { unsafe { syscall(5, p, 0, 0, 0) } }
}