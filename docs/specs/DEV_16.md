# 16. THE BRIDGE: TALKING TO THE KERNEL
### API LAYER // DEV-SPEC-16

AetherOS uses the **Neuro-Kernel Protocol (NKP)** to bridge the high-level Interface (React) and the low-level Engine (Rust).

## 1. THE SYSCALL FLOW
When you click a button in the UI, it sends a **JSON Packet** to the Kernel.

```typescript
// Interface Layer (TypeScript)
kernelBridge.exec(KernelCommand.FS_WRITE, { 
    path: '/home/hello.txt', 
    content: 'Hello from the UI!' 
});
```

## 2. THE RUST RECEPTION
The Kernel receives this, validates the **Intent**, and executes the binary command.

```rust
// Engine Layer (NanoRust)
fn handle_write_request(path_ptr, content_ptr) {
    // 1. Verify safety via Constitutional Judge
    // 2. Perform the physical write
    unsafe { vfs_write(path_ptr, content_ptr); }
}
```

## 3. WHY DECOUPLE?
By keeping the Interface and the Engine separate, we can change the look of AetherOS without ever touching the critical Rust code that keeps the system alive.

---
*BRIDGE v1.0 // CRYSTALLIZED PROTOCOL*