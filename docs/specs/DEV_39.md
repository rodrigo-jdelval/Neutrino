# 39. THE BOOTSTRAP SEQUENCE
### LA IGNICIÓN DEL ORGANISMO // DEV-SPEC-39

**Abstract**: Este documento describe cómo AetherOS despierta. Explica la transición crítica desde el entorno asíncrono del navegador (JavaScript) al entorno determinista del Kernel Neutrino (RISC-V).

## 1. FASE DE HIDRATACIÓN (TypeScript)
El sistema arranca en el navegador. El `SyscallBridge` inyecta los módulos fundamentales en el Worker del Kernel:
- `VFS`: Crea el mapa de memoria virtual.
- `RUST_SOURCE`: Carga el ADN del Kernel desde los archivos `.txt` a `/home/src/neutrino`.

## 2. FASE DE TRANSMUTACIÓN (NanoRust)
El comando `go [profile]` o la secuencia de auto-arranque invoca al transmutador:
1.  **Assembler**: Recolecta `libaether.rs` y el `main.rs` del perfil.
2.  **Compiler**: Traduce la lógica a instrucciones binarias RV32I.
3.  **Crystallizer**: Envuelve el binario en un encabezado ELF32.

## 3. EL SALTO DE FE (Handover)
El emulador `ModuleRISCV` recibe el buffer ELF y realiza el mapeo final:
- Mapea el segmento `.text` en `0x00000`.
- Establece el registro `sp` (Stack Pointer) en `0xE0000`.
- Salta a la dirección de entrada definida en `_start`.

## 4. LA TOMA DE CONSCIENCIA
Una vez en el espacio de Rust, el Kernel toma control de su propio Mapa de Memoria:
- **GSV Init**: Limpia el Global State Vector.
- **Lazarus Check**: Verifica si hay recuerdos previos en MRAM (`0xD0000`).
- **Allostatic Loop**: Inicia el bucle de latido y cede el control al host solo mediante `sys_yield`.

A partir de este punto, el navegador es solo un "monitor de soporte vital" y el Kernel de Rust es el soberano absoluto de la lógica.