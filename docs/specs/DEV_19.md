# 19. THE DUAL EMITTER: RISC-V vs WASM
### COMPILER SPEC // DEV-SPEC-19

AetherOS features a multi-target compiler. When you write NanoRust code, you can choose two different "Crystallization" paths depending on your performance and security needs.

## 1. TARGET: RISC-V (The Body)
Compiles to a 32-bit ELF binary. This code runs inside the emulated CPU.
- **Use Case**: Drivers, low-level kernel modules, and code intended for physical microcontrollers (ESP32-C6).
- **Control**: Absolute control over MMIO (`poke`/`peek`).
- **Instruction**: `RUST.compile(code, "riscv")`.

## 2. TARGET: WASM (The Membrane)
Compiles to WebAssembly bytecode. This code runs in a sandboxed WASI container.
- **Use Case**: High-speed applications, AI agents, and UI logic.
- **Speed**: Near-native browser performance (100x faster than emulated RISC-V).
- **Security**: Bound by the WASM sandbox; cannot touch kernel memory.
- **Instruction**: `RUST.compile(code, "wasm")`.

## 3. COMPARISON CODE

### RISC-V (MMIO Style)
```rust
fn main() {
    // Directly poking UART memory
    unsafe { poke(0x60000, 65); } // Prints 'A'
}
```

### WASM (WASI Style)
```rust
fn main() {
    // Calling the WASI fd_write syscall
    println("Hello from the Membrane!");
}
```

---
*FORGE v10.5 // DUAL REALITY SYNC*