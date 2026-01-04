# 18. THE 5-LAYER STACK: AETHER ARCHITECTURE
### ARCHITECTURAL PHILOSOPHY // DEV-SPEC-18

To ensure AetherOS survives until 2055, we use a decoupled 5-layer architecture.

## 1. THE ENGINE (Framework Layer)
The "Heart". This is the **Neutrino Kernel** running in the background worker. It manages memory, files, and hardware.

## 2. THE BRIDGE (API Layer)
The "Nerves". The **SyscallBridge** that allows the UI to talk to the Worker without blocking.

## 3. THE INTERFACE (Application Layer)
The "Eyes". The **React components** you see. They are purely visual and should contain zero critical logic.

## 4. THE CANVAS (Dashboard Layer)
The "Hand". The **Desktop environment** where windows and graphs are orchestrated.

## 5. THE DATA STORE (Persistence Layer)
The "Memory". **IndexedDB and OPFS** where your sovereignty is stored as immutable Merkle Trees.

---
*DECOUPLING IS THE PATH TO IMMORTALITY.*