# 10. THE ROSETTA STONE: COMPLETE ISA
### REFERENCIA TÉCNICA DE BAJO NIVEL // DEV-SPEC-10

Para reconstruir el sustrato Aether, un sistema debe implementar exactamente las 47 instrucciones base de RISC-V (RV32I) y los 10 primitivos de la Membrana WASM. Este es el "Alfabeto del Silicio".

## 1. REGISTROS RV32I (ABI Estandard)
NanoRust mapea las variables directamente a estos 32 registros de hardware:

| Registro | Alias | Rol en AetherOS |
| :--- | :--- | :--- |
| `x0` | `zero` | Cero cableado (Hardwired). |
| `x1` | `ra` | Dirección de retorno de función. |
| `x2` | `sp` | Puntero de Pila (Inicia en `0xE0000`). |
| `x10-x11` | `a0-a1` | Argumentos y Valor de Retorno. |
| `x17` | `a7` | **Syscall ID** (Usado por `ECALL`). |

## 2. EL ALFABETO DE INSTRUCCIONES
AetherOS utiliza un subconjunto optimizado para determinismo y baja latencia.

### 2.1 Aritmética y Lógica (R-Type)
Instrucciones procesadas en un solo ciclo de reloj virtual:
- **ADD / SUB**: Suma y resta de enteros de 32 bits.
- **SLL / SRL / SRA**: Desplazamientos de bits para criptografía y packing.
- **XOR / OR / AND**: Operaciones lógicas base para máscaras de bits.

### 2.2 Acceso a Memoria (I/S-Type)
El único puente entre el CPU y el Mapa de Memoria:
- **LW (Load Word)**: Lee 4 bytes desde la RAM al registro.
- **SW (Store Word)**: Escribe 4 bytes desde el registro a la RAM (MMIO).

## 3. EL PROTOCOLO DE LLAMADA A SISTEMA (ABI)
Cuando el código Rust invoca `yield` o un comando de hardware, emite una instrucción `ECALL`. El registro `a7` decide el destino:

```rust
// Ejemplo de Syscall 120 (Yield)
li a7, 120    // Cargar ID 120
ecall         // Trap al Kernel/Navegador
```

## 4. MAPEO DE COMPILACIÓN
| NanoRust | Ensamblador RISC-V (Crystallized) | Binario (Hex) |
| :--- | :--- | :--- |
| `let x = 5;` | `addi x18, x0, 5` | `0x00500913` |
| `poke(a, v);` | `sw x19, 0(x18)` | `0x01392023` |
| `yield;` | `ecall (con a7=120)` | `0x07800073` |

---
*REVISIÓN 9.38 // CÓDIGO ES MATERIA.*