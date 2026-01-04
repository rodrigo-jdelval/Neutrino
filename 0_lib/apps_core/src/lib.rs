#![no_std]
pub fn app_matrix(tick: u32) {
    if tick % 2 == 0 {
        let x = Drivers::rng_next() % 64;
        let y = Drivers::rng_next() % 64;
        plot_pixel(x, y, 0xFF00FF00);
    }
}