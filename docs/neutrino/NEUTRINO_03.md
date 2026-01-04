# CHAPTER 3: MEMORY MANAGEMENT (ARENAS)

**Abstract**: Neutrino implements a stratified memory model designed for a 30-year uptime. This chapter explores the **Global State Vector**, the **Bump Allocator** (Arena), and the mathematical rigor of 32-bit pointer alignment.

### 3.1 THE GLOBAL STATE VECTOR (GSV)
To eliminate the need for a dynamic symbol table, Neutrino partitions a "Safe Zone" in low memory (`0x38000`) where the kernel's most vital variables reside at fixed offsets.

| Offset | Semantic Unit | Description |
| :--- | :--- | :--- |
| `0x38000` | `HEAP_PTR` | Current ceiling of the linear allocator. |
| `0x38100` | `PROC_HEAD` | Pointer to the Linked List of active processes. |
| `0x38200` | `HORMONE_V` | 32-bit vector [Adr, Cort, Mel, Res]. |
| `0x38304` | `RTC_TICKS` | Monotonic 100Hz hardware clock. |

### 3.2 THE ARENA ALLOCATOR (BUMP)
Neutrino uses a **Monotonic Arena**. This is the fastest possible allocator ($O(1)$) but lacks an individual `free()` operation.

**Allocation Logic**:
$$ P_{new} = P_{curr} + 	ext{align}(Size) $$
$$ 	ext{If } P_{new} > STACK_{limit} 	o 	ext{Panic(OOM)} $$

To reclaim memory, the system performs a **"Homeostatic Reset"**: when the process queue is empty (PID 0 only), the `HEAP_PTR` is reset to its baseline (`0x50000`), effectively performing "Garbage Collection via Apocalypse."

### 3.3 ALIGNMENT CONSTRAINTS
RISC-V 32-bit hardware requires data to be aligned to a multiple of 4 bytes. Accessing a `u32` at address `0x50001` results in a **Load Address Misaligned** trap.

```rust
fn mem_align(n: u32) -> u32 {
    // Binary mask: (n + 3) & ~3
    // (n + 3) pushes to next multiple, & ~3 clears lower bits.
    (n + 3) & 0xFFFFFFFC
}
```