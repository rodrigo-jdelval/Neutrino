# 13. THE MMIO ATLAS: HARDWARE PORTS
### MAPA DE PERIFÉRICOS // DEV-SPEC-13

AetherOS no utiliza instrucciones de I/O especiales. Controlas el "mundo exterior" mediante **Memory Mapped I/O (MMIO)**. Escribir en estas direcciones es como conectar cables directamente a los pines de un chip.

## 1. REFERENCIA MAESTRA DE REGISTROS

| Rango de Memoria | Nombre | Función |
| :--- | :--- | :--- |
| `0x60000` | **UART_TX** | Puerto de Salida de Texto (Serial). |
| `0x60004` | **UART_RX** | Puerto de Entrada (Teclado). |
| `0x40000` - `0x43FFF` | **VRAM** | Framebuffer de Vídeo (16KB). |
| `0x38304` | **RTC** | Real Time Clock (Contador de Ticks). |

## 2. ARQUITECTURA DE VÍDEO (VGA 64x64)
El framebuffer es un array lineal de 4096 píxeles. Cada píxel ocupa 4 bytes (RGBA).

**Fórmula de Localización:**
$$ Dirección = 0x40000 + (Y \times 64 + X) \times 4 $$

### Mapa de Bits de un Píxel (Little Endian):
- **Byte 0**: Azul (0-255)
- **Byte 1**: Verde (0-255)
- **Byte 2**: Rojo (0-255)
- **Byte 3**: Alpha/Opacidad (0-255)

## 3. CONTROL DE TEXTO (UART)
El puerto serie es tu línea de vida. 

- **Escribir**: `poke(0x60000, caracter)` envía un byte al terminal.
- **Leer**: `peek(0x60004)` devuelve 0 si no hay teclas pulsadas, o el código ASCII si hay datos esperando.

## 4. EJEMPLO: DRIVER DE GRÁFICOS MINIMALISTA
```rust
fn draw_rect(x, y, w, h, color) {
    let mut curr_y = y;
    while curr_y < (y + h) {
        let mut curr_x = x;
        while curr_x < (x + w) {
            let addr = 0x40000 + (curr_y * 64 + curr_x) * 4;
            unsafe { poke(addr, color); }
            curr_x = curr_x + 1;
        }
        curr_y = curr_y + 1;
    }
}
```

## 5. REGLA DE ORO DE ALINEACIÓN
RISC-V requiere que las escrituras de 32 bits estén alineadas a 4 bytes. 
- **Correcto**: `poke(0x40000, c)`
- **Incorrecto**: `poke(0x40001, c)` -> Genera un `Load Address Misaligned Trap`.

---
*REVISIÓN 9.38 // EL HARDWARE ES SOLO MEMORIA.*