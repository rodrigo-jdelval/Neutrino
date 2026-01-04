# 03. POINTERS: THE DATA HOTEL
### MAESTRÍA EN EL SUBSTRATO // DEV-SPEC-03

En los lenguajes modernos, los punteros están ocultos. En NanoRust, son tu herramienta principal. Para programar AetherOS, debes convertirte en el **Conserje del Hotel**.

## 1. LA ANALOGÍA DEL HOTEL
Imagina el 1MB de RAM como un hotel con **1,048,576 habitaciones**.
- **La Dirección**: El número de habitación (ej. `0x50000`).
- **El Dato**: El huésped que se aloja en esa habitación.
- **El Puntero**: Un papel con el número de habitación escrito.

## 2. POKE & PEEK (Las Operaciones Atómicas)
- **POKE**: "Pon a este huésped en la habitación X".
- **PEEK**: "Dime quién está en la habitación X".

## 3. GEOMETRÍA DE DATOS: ARITMÉTICA
Cuando guardas una lista de números, ocupan habitaciones contiguas. Como cada número ocupa **4 habitaciones** (32 bits = 4 bytes), para encontrar al siguiente huésped sumas 4.

| Índice | Cálculo de Dirección | Dirección Resultante |
| :--- | :--- | :--- |
| Huésped 0 | `Base + (0 * 4)` | `0x50000` |
| Huésped 1 | `Base + (1 * 4)` | `0x50004` |
| Huésped 2 | `Base + (2 * 4)` | `0x50008` |

```mermaid
graph LR
    P[Puntero Base: 0x50000] --> R0[Room 0]
    R0 --> R1[+4 bytes]
    R1 --> R2[+4 bytes]
    style R0 fill:#4cc9f0,color:#000
    style R1 fill:#4cc9f0,color:#000
    style R2 fill:#4cc9f0,color:#000
```

## 4. LA HABITACIÓN PROHIBIDA (NULL)
La **Habitación 0x00000** es la suite del fundador (el código del Kernel). Si intentas meter a un huésped allí (un "Null Pointer Dereference"), el Juez Cognitivo detectará la profanación e iniciará la **Apoptosis Celular** (matará tu proceso instantáneamente).

## 5. EJEMPLO: CREANDO UN ARRAY MANUAL
```rust
fn main() {
    let array_start = 0x50000; // Iniciamos en el HEAP
    
    let mut i = 0;
    while i < 10 {
        let current_room = array_start + (i * 4);
        unsafe {
            poke(current_room, i * 10); // Guardamos 0, 10, 20...
        }
        i = i + 1;
    }
    
    println("Array inicializado en el sustrato.");
}
```

---
*LA DISCIPLINA ES EL PRECIO DE LA SOBERANÍA.*