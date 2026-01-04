#![no_std]
#![no_main]
include! "../../0_lib/tmr_core/src/lib.rs";
include! "../../0_lib/sbc_core/src/lib.rs";
include! "../../0_lib/drivers_core/src/lib.rs";
include! "../../0_lib/vga_core/src/lib.rs";
include! "../../0_lib/neurite_core/src/lib.rs";
include! "../../0_lib/immune_core/src/lib.rs";
include! "../../0_lib/health_core/src/lib.rs";
include! "../../0_lib/thermal_core/src/lib.rs";
include! "../../0_lib/proprio_core/src/lib.rs";
include! "../../0_lib/memory_core/src/lib.rs";
include! "../../0_lib/persistence_core/src/lib.rs";
include! "../../0_lib/coordination_core/src/lib.rs";
include! "../../0_lib/judgment_core/src/lib.rs";
include! "../../0_lib/vfs_core/src/lib.rs";
include! "../../0_lib/identity_core/src/lib.rs";
include! "../../0_lib/fractal_core/src/lib.rs";
include! "../../0_lib/offload_core/src/lib.rs";
include! "../../0_lib/mycelial_core/src/lib.rs";
include! "../../0_lib/wasm_core/src/lib.rs";
include! "../../0_lib/ipc_core/src/lib.rs";
include! "../../0_lib/process_core/src/lib.rs";
include! "../../0_lib/apps_core/src/lib.rs";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    Drivers::init();
    let mut bio = HormonalBus::new();
    let mut mem = MemoryManager::new();
    
    clear_screen(0xFF050a14);
    Drivers::uart_write(65); Drivers::uart_write(69); Drivers::uart_write(84); Drivers::uart_write(72); Drivers::uart_write(69); Drivers::uart_write(82); Drivers::uart_write(10);

    let mut tick = 0;
    loop {
        tick += 1;
        Drivers::rtc_tick();
        
        if tick % 100 == 0 {
            app_matrix(tick);
            plot_pixel(62, 62, if (tick/100)%2==0 { 0xFF00FF00 } else { 0xFF003300 });
        }
        
        unsafe { sys_yield(); }
    }
}