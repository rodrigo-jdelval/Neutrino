# 40. NANORUST: LANGUAGE SPECIFICATION (v13.0)
### THE SOVEREIGN CONTRACT // DEV-SPEC-40

**NanoRust** es un dialecto estricto y minimalista de Rust, diseñado específicamente para la **Compilación Autárquica** (Self-Hosting) dentro de entornos con recursos limitados (<1MB RAM).

No es Rust estándar. Es un lenguaje de sistemas de "Paso Único" que sacrifica abstracciones de tiempo de compilación (como el Borrow Checker) a cambio de un compilador diminuto (<10KB) que puede ejecutarse dentro del propio Kernel.

---

## 1. MODELO DE MEMORIA UNIFICADO

NanoRust trata la memoria como un array lineal de bytes (`u8`), pero opera lógicamente con palabras de 32 bits (`u32`).

### 1.1 Tipos Primitivos
En NanoRust, **todo es un `u32`**. No existen tipos `u8`, `i16` o `bool` reales en los registros.
*   `bool`: `0` (False), `>0` (True).
*   `char`: Valor ASCII/Unicode (32-bit).
*   `ptr`: Dirección de memoria (índice en el array RAM).

### 1.2 Estructuras (Structs) & Layout
A partir de la v13.0, los `structs` definen un diseño de memoria contiguo. El compilador resuelve los accesos a campos (`.`) calculando offsets estáticos en tiempo de compilación.

```rust
struct Point {
    x: u32, // Offset +0
    y: u32, // Offset +4
    z: u32  // Offset +8
}
// Sizeof(Point) = 12 bytes
```

**Instanciación (Sugar Sintáctico):**
```rust
let p = Point { x: 10, y: 20, z: 30 };
// Se compila a:
// 1. ptr = mem_alloc(12);
// 2. poke(ptr + 0, 10);
// 3. poke(ptr + 4, 20);
// 4. poke(ptr + 8, 30);
// 5. let p = ptr;
```

---

## 2. SINTAXIS Y GRAMÁTICA

### 2.1 Bloques de Implementación (Impl)
NanoRust soporta métodos asociados mediante *Name Mangling* simple. No hay `self` real; el primer argumento se pasa implícitamente como puntero.

```rust
impl Point {
    fn new(x: u32, y: u32) -> Point {
        // En v13, retornamos la estructura construida
        return Point { x: x, y: y, z: 0 };
    }

    fn sum(self) -> u32 {
        // Acceso a campos via self pointer
        return self.x + self.y;
    }
}

// Uso:
let p = Point::new(10, 20);
let s = p.sum(); // Se compila a: Point_sum(p)
```

### 2.2 Control de Flujo
El compilador genera automáticamente las estructuras de salto adecuadas para cada backend.

*   `if cond { ... } else { ... }`
*   `while cond { ... }`
*   `loop { ... }` (Bucle infinito optimizado)

**Diferencia de Backend:**
*   **RISC-V**: Genera instrucciones de salto condicional (`BGE`, `JAL`).
*   **WASM**: Genera bloques estructurados (`block`, `loop`, `br_if`).

---

## 3. ARQUITECTURA DE EMISIÓN DUAL (POLYGLOT)

El mismo código fuente NanoRust produce dos binarios radicalmente diferentes según el flag `target`.

### 3.1 Target: RISC-V (Silicio)
*   **Output**: ELF 32-bit (Little Endian).
*   **Uso**: Kernel, Drivers, IoT (ESP32).
*   **Acceso HW**: Directo vía `poke(0xF00...)`.
*   **Stack**: Gestionado manualmente en registro `x2` (SP).

### 3.2 Target: WASM (Membrana)
*   **Output**: Módulo WebAssembly (.wasm).
*   **Uso**: Aplicaciones de Usuario (Apps), Agentes IA.
*   **Acceso HW**: Prohibido. Solo vía Syscalls (WASI).
*   **Stack**: Máquina de pila virtual. Las variables locales se mapean a `local.get/set`.

---

## 4. INTRÍNSECOS DEL SISTEMA (SDK)

Funciones integradas en el compilador para operaciones de bajo nivel.

| Función | Descripción | Coste (Ciclos) |
| :--- | :--- | :--- |
| `poke(addr, val)` | Escribe 32-bits en RAM. | 1 |
| `peek(addr)` | Lee 32-bits de RAM. | 1 |
| `pokeb(addr, val)` | Escribe 8-bits (byte). | 1 |
| `sys_yield()` | Cede el control al Host (Browser/OS). | Variable |
| `syscall(id, ...)` | Invoca una función del Kernel (ABI). | Variable |

---

## 5. LIMITACIONES DEL COMPILADOR (SINGLE-PASS)

Debido a que el compilador debe caber en 1MB de RAM junto con el Kernel, tiene limitaciones arquitectónicas importantes:

1.  **Sin "Lookahead" de Tipos**: Debes definir las estructuras *antes* de usarlas.
2.  **Backpatching de Funciones**: Puedes llamar a funciones definidas más abajo en el archivo, pero el compilador insertará un *stub* y lo parcheará al final. Esto impide cierto inlining.
3.  **No hay Genéricos**: `List<T>` no existe. Usa `u32` (void*) y casting manual.
4.  **No hay Borrow Checker**: Si liberas memoria que estás usando, el sistema crasheará. La seguridad es responsabilidad del programador (o de la IA que escribe el código).

---

## 6. LA VISIÓN: EL BINARIO TRANSLÚCIDO

En el futuro (Phase 21), NanoRust emitirá un **Binario Translúcido**: un archivo que contiene *ambas* secciones (RISC-V y WASM). El cargador del Kernel decidirá qué sección ejecutar basándose en si el hardware anfitrión es silicio físico o un navegador web, garantizando la ejecución óptima en cualquier sustrato de 2055.

*DOCUMENTO DE REFERENCIA OFICIAL v13.0*