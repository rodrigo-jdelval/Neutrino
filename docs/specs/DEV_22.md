# DEV_22: SEMANTIC OFFLOADING
### Virtual Translucency Engine // Phase 15.2

El **Semantic Offloading** es el mecanismo que permite a un proceso SIP (Software Isolated Process) que corre en el emulador RISC-V delegar tareas de alta intensidad computacional al Host (JavaScript/Nativo).

## 1. El ID de Syscall
Se ha reservado el ID de syscall `250` (`SYS_SEMANTIC_OFFLOAD`) para esta operación.

## 2. Operaciones Soportadas
Actualmente, el motor de transimisión soporta:

| Opcode | Operación | Implementación Host |
| :--- | :--- | :--- |
| `HASH_SHA256` | Criptografía | `crypto.subtle.digest` |
| `VGA_BLIT_FAST` | Gráficos | Manipulación directa de `SharedArrayBuffer` |
| `AI_MINI_INFER` | Inferencia | Modelo cuantizado local (WebLLM) |

## 3. Ejemplo de llamada (ABI)
El registro `a0` contiene el puntero al nombre de la operación, y `a1` el puntero a los datos.
```rust
// Código NanoRust
let hash = sys_offload("HASH_SHA256", "data_to_hash");
```