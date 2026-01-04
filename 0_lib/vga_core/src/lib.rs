#![no_std]
pub const VRAM_BASE: usize = 0x40000;
pub fn plot_pixel(x: u32, y: u32, color: u32) {
    if x < 64 && y < 64 { unsafe { poke(VRAM_BASE + ((y * 64 + x) as usize * 4), color as usize); } }
}
pub fn clear_screen(color: u32) {
    let mut i = 0; while i < 4096 { unsafe { poke(VRAM_BASE + (i * 4), color as usize); } i += 1; }
}
pub fn draw_text(x: u32, y: u32, s: &str, color: u32) {
    // Simplified text renderer
}