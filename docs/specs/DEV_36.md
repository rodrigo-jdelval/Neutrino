# 36. FUNCTIONAL MAPPING: THE RUST GENOME
### TRAZABILIDAD DE SUBSISTEMAS // DEV-SPEC-36

**Abstract**: Este apéndice sirve como guía de navegación para ingenieros de sustrato, mapeando las capacidades biológicas y estratégicas de AetherOS con sus respectivos módulos de código fuente en Rust dentro de la estructura `kernel/neutrino`.

## 1. MAPA DE LÓGICA DE SUPERVIVENCIA (DNA)

| Funcionalidad | Módulo Rust (Ruta Relativa) | Clase / Estructura Principal |
| :--- | :--- | :--- |
| **Allostatic Hormonal Bus** | `0_lib/neurite_core/src/lib.rs` | `HormonalBus` |
| **Nash Equilibrium / Bidding** | `0_lib/coordination_core/src/lib.rs` | `CoordinationEngine` |
| **Triple Modular Redundancy** | `0_lib/tmr_core/src/lib.rs` | `Triplet` |
| **Membrana de Aislamiento** | `0_lib/wasm_core/src/lib.rs` | `WasmInterpreter` |
| **Metacognitive Health** | `0_lib/health_core/src/lib.rs` | `HealthMonitor` |
| **Soberanía de Identidad** | `0_lib/identity_core/src/lib.rs` | `IdentityPrism` |
| **Fractalización Hydra** | `0_lib/fractal_core/src/lib.rs` | `Fractalizer` |
| **Protocolo Lazarus** | `0_lib/persistence_core/src/lib.rs` | `Lazarus` |
| **Inmunidad Entrópica** | `0_lib/immune_core/src/lib.rs` | `ImmuneSystem` |
| **Interocepsión / Proprio** | `0_lib/proprio_core/src/lib.rs` | `BodySchema` |
| **Súper-Ego Constitucional** | `0_lib/judgment_core/src/lib.rs` | `ConstitutionalJudge` |
| **VFS Sináptico** | `0_lib/vfs_core/src/lib.rs` | `SynapseVfs` |
| **Process Management** | `0_lib/process_core/src/lib.rs` | `ProcessManager` |
| **Visual Arts (Matrix)** | `0_lib/apps_core/src/lib.rs` | `app_matrix` |

## 2. MAPA DE INFRAESTRUCTURA (SILICIO VIRTUAL)

| Funcionalidad | Módulo Rust (Ruta Relativa) | Descripción |
| :--- | :--- | :--- |
| **Orquestación Allostática** | `4_profiles/kernel/src/main.rs` | Bucle principal (Shell, Scheduler, Biology). |
| **Hardware Drivers** | `0_lib/drivers_core/src/lib.rs` | UART (IO), RNG (Entropy), RTC (Time). |
| **Memory Gating** | `0_lib/memory_core/src/lib.rs` | Gestión de heap y Necrotic Skipping. |
| **Regulación Térmica** | `0_lib/thermal_core/src/lib.rs` | Throttling preventivo por estrés. |
| **VGA Rendering** | `0_lib/vga_core/src/lib.rs` | Escritura directa en VRAM (MMIO) y Fuentes 3x5. |
| **SBC Encoding** | `0_lib/sbc_core/src/lib.rs` | Serialización binaria de hormonas. |
| **Mycelial Link** | `0_lib/mycelial_core/src/lib.rs` | Comunicación P2P y Sensory Borrowing. |
| **Semantic Offloading** | `0_lib/offload_core/src/lib.rs` | Puente hacia syscalls complejas del host. |
| **Inter-Process Comm** | `0_lib/ipc_core/src/lib.rs` | Mailbox Zero-Copy y Señales. |

## 3. NOTA DE DISEÑO
Todos los módulos bajo `0_lib` están diseñados como crates `#![no_std]`. El **Virtual Linker** del comando `go` inyecta estos archivos recursivamente en tiempo de compilación para generar un binario monolítico compatible con la arquitectura RV32I emulada.