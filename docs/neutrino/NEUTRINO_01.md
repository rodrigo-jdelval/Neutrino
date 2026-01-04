# CHAPTER 1: THE BARE METAL ENVIRONMENT

**Abstract**: This chapter details the "Birth of the Kernel." We examine the transition from the emulator's `LOAD` event to the execution of the first instruction at `_start`, the management of the stack pointer, and the #![no_std] constraints.

### 1.1 THE EVENT HORIZON (_start)
In a hosted OS, `main()` is called after the C-runtime (CRT) has initialized. In Neutrino, there is no runtime. The hardware (emulator) jumps blindly to the address specified in the ELF `e_entry` header.

```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Initial State: Registers are random, Stack is uninitialized.
    // 2. Hardware Wake: UART output enabled.
    // 3. Memory Mapping: Setting the HEAP_PTR.
    // 4. Entering the Allostatic Loop.
    loop { sys_yield(); }
}
```

### 1.2 STACK TOPOLOGY & THE SPINAL CORD
The Stack Pointer (`sp` / `x2`) is the most critical register in the bare-metal environment. It is initialized at the top of the memory map (`0xE0000`) and grows downwards towards the Heap.

**Stack Collision Logic**:
The kernel monitors the distance between the `HEAP_TOP` and the `sp`. If $SP - HEAP_{top} < 1024$ bytes, the kernel triggers a **Stack Guard Exception**, as a collision would result in non-deterministic logic corruption.

### 1.3 THE BOOT SEQUENCE (PHASES)
- **POWER_ON**: Emulator Reads Buffer
- **ELF_LOAD**: Clear .BSS Segment
- **HAL_WAKE**: Init UART/VGA
- **MEM_MAP**: Set HEAP_PTR (0x50000)
- **SPAWN_IDLE**: PID 0 Created
- **USER_SHELL**: PID 3 Active -> System Ready

### 1.4 THE PANIC SUBSTRATE
When the kernel fails, it must fail "beautifully" (visibly). The `panic_handler` is the only part of the system allowed to bypass the Virtual File System (VFS) and talk directly to the VGA hardware using raw pointers.