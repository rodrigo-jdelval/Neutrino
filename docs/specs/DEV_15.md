# 15. PARADIGM SHIFT: RUST vs NANORUST
### FILOSOFÍA DE LA AUTARQUÍA // DEV-SPEC-15

NanoRust es a Rust lo que un bisturí es a un brazo robótico industrial. Hemos eliminado la complejidad para ganar **Soberanía**.

## 1. COMPARATIVA DE HORIZONTES

| Característica | Standard Rust (`rustc`) | NanoRust (AetherOS) |
| :--- | :--- | :--- |
| **Borrow Checker** | Estático y Estricto. | **Humano.** Tú gestionas los punteros. |
| **Tamaño Binario** | > 100MB (Compilador). | **< 100KB** (Compilador). |
| **Abstracción** | Zero-cost abstractions. | **Zero abstraction.** Mapeo literal. |
| **Gestión de Memoria** | RAII / Ownership. | Punteros Crudos / `poke` y `peek`. |
| **Soberanía** | Depende de LLVM. | **Autárquico.** Escrito desde cero. |

## 2. EL COMPROMISO DE LA SEGURIDAD
Mientras que Rust estándar garantiza seguridad en tiempo de compilación, NanoRust traslada la responsabilidad a la **Membrana WASM** y al **Juez Cognitivo**.

- **Rust**: "Este código no compilará si es inseguro."
- **NanoRust**: "Este código compilará, pero la Membrana lo detendrá en runtime si intenta profanar el Kernel."

## 3. CÓDIGO COMPARATIVO: ACCESO A HARDWARE

### En Rust Estándar (Complejo/Dependiente):
```rust
use std::ptr;
fn write_uart(c: u8) {
    unsafe { ptr::write_volatile(0x60000 as *mut u8, c); }
}
```

### En NanoRust (Soberano/Directo):
```rust
fn write_uart(c) {
    unsafe { poke(0x60000, c); }
}
```

## 4. ¿POR QUÉ ESTA DIVERGENCIA?
Un sistema que requiere una conexión a internet para descargar una cadena de herramientas de 2GB no es un sistema libre. AetherOS utiliza NanoRust porque permite que el **Ouroboros Loop** ocurra en cualquier dispositivo, permitiendo que el OS se reconstruya a sí mismo en medio de un desierto tecnológico.

---
*REVISIÓN 1.0 // LA LIBERTAD TIENE UN PRECIO: EL RIGOR MANUAL.*