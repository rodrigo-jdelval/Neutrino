#![no_std]
#![no_main]
include! "../../1_hal/src/lib.rs";
include! "../../2_chips/virt_riscv/src/lib.rs";
include! "../../3_boards/virt_browser/src/lib.rs";
include! "../../0_lib/neurite_core/src/lib.rs";
include! "../../0_lib/immune_core/src/lib.rs";
include! "../../0_lib/health_core/src/lib.rs";
include! "../../0_lib/mycelial_core/src/lib.rs";
include! "../../0_lib/proprio_core/src/lib.rs";
include! "../../0_lib/drivers_core/src/lib.rs"; // Needed for Board Init

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    let serial = SerialDriver::new();
    let mut bio = HormonalBus::new();
    let mut immune = ImmuneSystem::new();
    
    serial.write_char(83); serial.write_char(69); serial.write_char(78); serial.write_char(83); serial.write_char(10);
    
    loop {
        bio.update(0);
        immune.scan(255);
        
        if bio.get_adr() > 200 { serial.write_char(33); } 
        else if bio.get_mlt() > 200 { sys_yield(); }
        else { serial.write_char(46); }
        
        sys_yield();
    }
}