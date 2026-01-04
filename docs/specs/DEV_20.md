# 20. THE EMULATOR ENGINE: LEGACY SYMBIOSE
### RUNTIME SPEC // DEV-SPEC-20

The **Emulator** app in AetherOS is more than a game console; it is a bridge to the deterministic past. It allows the Sovereign Kernel to interact with architectures that existed before the cognitive revolution.

## 1. THE RISC-V VM
The primary emulator implements the **RV32I** instruction set. It acts as the physical "Substrate" for the kernel when running on non-WASM hardware.
- **Clock**: 1.2GHz (Simulated).
- **RAM**: 1MB mapped flat.
- **Display**: 64x64 RGBA Framebuffer.

## 2. LEGACY VIRTUALIZATION
AetherOS can host entire legacy operating systems (Linux, Windows 95, DOS) inside its desktop. This is done by mounting remote images and providing them with virtual hardware descriptors.

## 3. HOW TO BOOT AN ARTIFACT
In the **Terminal (ash)**, you can manually trigger the emulator for any ELF file:
```bash
# Compile source to ELF
co /home/my_app.rs 

# Boot into the Virtual Machine
rv32 /home/my_app.elf
```

## 4. MULTI-CONSOLE MODE
You can open multiple emulators simultaneously. The Aether Scheduler will treat each VM as a high-priority process, ensuring smooth execution even when multitasking between a Linux shell and a NanoRust kernel.

---
*VIRTUALIZATION v4.2 // ARCHIVAL CONTINUITY*