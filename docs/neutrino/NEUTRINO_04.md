# CHAPTER 4: THE BIOLOGICAL SCHEDULER

**Abstract**: Neutrino replaces traditional time-sharing with a **Metabolic Model**. CPU cycles are not free; they are purchased with **Tokens**. This chapter details the Process Control Block (PCB) structure and the equations governing the Economic Scheduler.

### 4.1 THE PROCESS CONTROL BLOCK (PCB)
The PCB is the system's "Gene," storing the identity and economic status of a process in 44 bytes.

| Offset | Name | Role |
| :--- | :--- | :--- |
| `+0` | `PID` | Process ID (Atomic counter). |
| `+4` | `STATE` | 0:Ready, 1:Running, 2:Zombie, 3:Bankrupt. |
| `+8` | `PC` | Saved Instruction Pointer. |
| `+28` | `BUDGET` | Current Token balance. |
| `+32` | `METAB` | Tax rate (Tokens per tick). |
| `+36` | `MAIL` | Single-slot IPC Mailbox. |

### 4.2 THE TOKENOMICS FORMULA
Every process must pay a **Metabolic Tax** to remain in the `READY` queue. This creates a natural "Darwinian" pressure against infinite loops.

$$ Budget_{t+1} = Budget_t - (	ext{Metabolism} 	imes 	ext{StressFactor}) $$

Where **StressFactor** is derived from the global **Cortisol** level:
- If $Cortisol < 100 implies 	ext{StressFactor} = 1.0$
- If $Cortisol > 200 implies 	ext{StressFactor} = 2.5$ (Hyper-inflation of thought cost).

### 4.3 BANKRUPTCY & CRYOSTASIS
When $Budget le 0$, the process enters **Bankruptcy**. It is not killed (Apoptosis), but frozen (Cryostasis). The scheduler will never pick a Bankrupt process unless:
1. The user provides **Attention** (Injection of 1000 tokens).
2. An **Adrenaline Spike** occurs (System-wide emergency wake).