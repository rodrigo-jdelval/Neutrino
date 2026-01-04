# 37. THE SOVEREIGN SDK: PROGRAMMING THE ORGANISM
### INTERFAZ DE DESARROLLO DE SUSTRATO // DEV-SPEC-37

**Abstract**: El SDK de AetherOS es el puente entre el código fuente del desarrollador y la fisiología del Kernel Neutrino. Define cómo un programa debe solicitar recursos, ceder ciclos y comunicarse con la red micelial.

## 1. MACROS DE SISTEMA (INTRÍNSECOS)
NanoRust expone funciones de bajo nivel que se mapean directamente a instrucciones RISC-V.

| Macro | Equivalente ASM | Propósito |
| :--- | :--- | :--- |
| `poke(addr, val)` | `sw rs2, 0(rs1)` | Escribe 32-bits en el mapa MMIO. |
| `peek(addr)` | `lw rd, 0(rs1)` | Lee 32-bits de una dirección física. |
| `pokeb(addr, val)` | `sb rs2, 0(rs1)` | Escribe un byte (8-bits). |
| `peekb(addr)` | `lbu rd, 0(rs1)` | Lee un byte (8-bits). |
| `sys_yield()` | `ecall (a7=120)` | Cede el control para mantenimiento allostático. |

## 2. ABI DE LLAMADAS AL SISTEMA (SYSCALLS)
Para interactuar con servicios complejos, se utiliza el registro `a7` (x17) como selector de función.

| ID (a7) | Nombre | Argumentos | Retorno |
| :--- | :--- | :--- | :--- |
| **1** | `SYS_EXIT` | `a0: code` | - |
| **3** | `SYS_READ` | `a0: fd, a1: buf, a2: len` | Bytes leídos |
| **4** | `SYS_WRITE` | `a0: fd, a1: buf, a2: len` | Bytes escritos |
| **5** | `SYS_OPEN` | `a0: path_ptr, a1: flags` | File Descriptor |
| **200** | `SYS_SECRETE` | `a0: hormone_id, a1: amount` | Nuevo nivel |
| **250** | `SYS_OFFLOAD` | `a0: opcode, a1: data_ptr` | Resultado |

## 3. TIPOS BASE Y ESTRUCTURAS
Ubicados en `0_lib/neutrino_core`, estos tipos aseguran la compatibilidad binaria:

```rust
// El resultado estándar para operaciones fallibles
pub type Result<T> = core::result::Result<T, usize>;

// Errores comunes del sustrato
pub enum Error {
    SubstrateExhausted = 1, // OOM o Falta de espacio
    DissonanceDetected = 2, // Rechazo por el Juez Constitucional
    ResonanceLoss      = 3, // Fallo en el Mycelial Link
}
```

## 4. EL CONTRATO DE RESONANCIA
Toda aplicación del SDK debe implementar el bucle de cortesía:
1.  **Sense**: Leer el estado de Adrenalina/Cortisol.
2.  **Act**: Ejecutar lógica proporcional al presupuesto de tokens.
3.  **Yield**: Invocar `sys_yield()` para permitir la sanación del sustrato.

Ignorar este contrato resulta en la degradación de la reputación del proceso y eventual **Apoptosis** forzada por el planificador.