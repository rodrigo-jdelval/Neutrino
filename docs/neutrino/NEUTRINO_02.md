# CHAPTER 2: THE HARDWARE ABSTRACTION LAYER (HAL)

**Abstract**: The HAL serves as the translation layer between the abstract logic of the kernel and the physical (simulated) reality of the hardware. This chapter details the implementation of Memory-Mapped I/O (MMIO) for the UART console and the VGA framebuffer, utilizing intrinsic pointer manipulation to bridge the Guest (WASM) and Host (Browser) worlds.

### 2.1 INTRINSIC MEMORY ACCESS (MMIO)

In a standard OS, drivers communicate via specific bus protocols (PCIe, USB). In AetherOS's emulated RISC-V environment, we use **Memory Mapped I/O**. Specific memory addresses do not point to RAM cells, but are "hot-wired" to the Emulator's event loop.

Reading or writing to these addresses triggers a side-effect in the Host Browser.

| Device | Address | Direction | Function |
| :--- | :--- | :--- | :--- |
| **VGA** | `0x40000` | Write-Only | Writes pixel data to the HTML5 Canvas. |
| **UART** | `0x60000` | Read/Write | Sends text to the Terminal / Reads keyboard input. |

### 2.2 THE VGA DRIVER (DIRECT FRAMEBUFFER)

The Video Graphics Array (VGA) implementation uses a linear framebuffer starting at `0x40000`. The resolution is fixed at **64x64 pixels** to align with the "Nano" aesthetic and minimize the VRAM bandwidth cost during the `postMessage` transfer between the Worker and the Main Thread.

**Pixel Addressing Formula:**
$$ Address = Base + ((y \times Width + x) \times 4) $$

Where 4 represents the bytes per pixel (RGBA 32-bit, Little Endian).

```rust
// kernel/src/graphics.rs

fn hal_vga_plot(x, y, color) {
    // Clipping
    if x < 64 {
        if y < 64 {
            let offset = (y * 64 + x) * 4;
            let addr = 0x40000 + offset;
            unsafe { poke(addr, color); }
        }
    }
}
```

### 2.3 THE FONT ENGINE (BIT-BANGING)

To render text during a Kernel Panic—before the Virtual File System (VFS) or assets are loaded—Neutrino requires a hardcoded bitmap font embedded directly in the `.text` segment.

We utilize a **15-bit Micro-Font**. Each character is a 3x5 grid encoded into a single `u16` integer.

*   **Example 'A'**: `0x2BED`
*   **Binary**: `010 101 111 101 101`