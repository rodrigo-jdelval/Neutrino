# CHAPTER 5: THE SWARM (IPC)

**Abstract**: Isolation without communication is autism. For the Neutrino microkernel to function as a "Swarm" of agents, it requires a robust Inter-Process Communication (IPC) mechanism. This chapter details the implementation of the **Synchronous Mailbox Protocol**, a minimalist approach to message passing that avoids the complexity of queues and sockets.

### 5.1 THE MAILBOX ARCHITECTURE

Neutrino eschews complex ring-buffers or dynamic sockets in favor of a **Zero-Copy, Single-Slot Mailbox** system located directly within the PCB.

*   **Location**: `PCB_Base + 36` (Offset 0x24).
*   **Capacity**: 1 Message (32-bit Integer or Pointer).
*   **Protocol**: Non-blocking Send / Polling Receive.

**Why Single Slot?**
In a constrained cognitive system, buffering messages hides latency and allows backpressure to accumulate unnoticed. By limiting the mailbox to one slot, we enforce immediate handling of state changes. If the receiver is overwhelmed, the sender knows immediately.

### 5.2 THE SEND/RECV SYSCALLS

The IPC system relies on two atomic operations implemented in `ipc.rs`:

#### `ipc_send(pid, data)`
1.  Scans the Process List to find the target PCB.
2.  Peeks at the target's Inbox offset.
3.  **If != 0 (Full)**: Returns `ERROR_BUSY`. The sender must retry later or drop the message.
4.  **If == 0 (Empty)**: Pokes the `data` into the slot. Returns `SUCCESS`.

#### `ipc_recv()`
1.  Peeks at the current process's Inbox.
2.  **If != 0**: Reads the data, then immediately Pokes `0` to clear the slot. Returns data.
3.  **If == 0**: Returns `0` (No message).

### 5.3 THE LISTENER PATTERN (PID 2)

We implement a dedicated system process (PID 2), the **Listener Service**. This process is spawned immediately after the kernel boots. It runs an infinite loop that polls its mailbox and acts as a central event bus.

```rust
// Simplified Service Logic
fn service_listener() {
    loop {
        let msg = ipc_recv();
        if msg != 0 {
            // React to message (e.g., log to UART)
            hal_uart_print("MSG RECVD");
        }
        sys_yield(); // Be polite, let others run
    }
}
```