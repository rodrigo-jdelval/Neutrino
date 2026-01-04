# CHAPTER 6: THE MIDORI MICRO-ARCHITECTURE
### Language-Based Isolation & Capabilities

**Abstract**: Neutrino transcends the legacy UNIX process model by adopting the tenets of Project Midori. We replace hardware-enforced address spaces with **Language-Level Safety** and replace identity-based ACLs with **Object Capabilities**.

### 6.1 BEYOND THE MMU (SIPs)
In traditional OSs, context switching between processes is expensive because the CPU must flush the TLB and swap page tables to prevent Process A from reading Process B's memory.

Neutrino uses **Software Isolated Processes (SIPs)**. Because the NanoRust compiler and the WASM verifier prove that a program cannot perform raw pointer arithmetic outside its own allocated buffers, the Kernel can run all processes in a **Single Flat Address Space**.

- **Performance**: Context switching cost drops from ~5000 cycles to ~10 cycles.
- **Density**: We can run thousands of micro-services on an ESP32-C6 that would normally only support a few dozen threads.

### 6.2 OBJECT CAPABILITIES (OCAPs)
In AetherOS, "Root" doesn't exist. Authority is not tied to a user, but to the possession of a **Token**.

When a process is spawned, it is given a set of opaque handles (Capabilities).
```rust
// Example: A SIP requesting a sensor capability
let temp_handle = sys_request_cap("dev:thermal");
```
If the process does not have the handle, the code to interact with that hardware is literally unreachable. This is **Security by Design**, not by policy.

### 6.3 FAIL-FAST (ERROR ABANDONMENT)
Midori's most radical lesson was that "Error Recovery" is often the cause of more bugs. Neutrino implements **Fail-Fast Abandonment**:

1. **Detection**: A SIP violates an invariant (e.g., array out of bounds).
2. **Abandonment**: The Kernel instantly halts the SIP. It does not run destructors or try to save state.
3. **Reclamation**: The process's memory (which is a discrete cell in the single address space) is zeroed.
4. **Resurrection**: The parent process receives a `SIG_ABANDON` and decides whether to restart the SIP from its immutable genetic source.

### 6.4 NON-BLOCKING KERNEL
There is no `sleep()` or `wait()` in the Neutrino core. Every syscall returns a **Future**. The scheduler is a reactor that only grants CPU time to processes whose asynchronous dependencies (Sensors, Network, IPC) have reached a state of **Semantic Resonance**.