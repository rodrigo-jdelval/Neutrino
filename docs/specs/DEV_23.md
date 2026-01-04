# DEV_23: SDK DE NANORUST (AETHER CORE)
### Especificación de Inteligencia // v21.0 (Binary NKP)

AetherOS implementa un modelo de pensamiento estratificado para maximizar la resiliencia y la eficiencia energética.

## 1. El Protocolo NKP Binario (Fast-Path)
Para acciones frecuentes y de bajo riesgo, el SDK ofrece `npu::reflex_op()`. Esta función evita el parseo de lenguaje natural y se comunica con el Kernel mediante OpCodes de 32 bits.

- **Ventaja**: Cero latencia de inferencia, cero consumo de tokens de frontera.
- **Uso**: Sanación de sustrato, optimización de flujos, sincronización de red.

## 2. Abstracción NPU (Sovereign AI)
El SDK no se vincula a ningún proveedor de modelos. Utiliza niveles de **Resonancia Cognitiva**:

| Nivel | Sustrato | Latencia | Propósito |
| :--- | :--- | :--- | :--- |
| **Reflex** | NKP Binario / Seed | < 1ms | Estabilidad y Supervivencia. |
| **Reason** | Frontier Link | > 500ms | Planificación y Síntesis. |

## 3. Ejemplo: Reacción en Tiempo Real
```rust
use aether::npu::{reflex, ReflexOp};

fn handle_critical_stress() {
    // Reacción "medular" instantánea (OpCode 0x01)
    npu::reflex(ReflexOp::HealSubstrate);
    log!("Substrato sanado vía vía rápida binaria.");
}
```

## 4. Opcodes Definidos
| OpCode | Enum Variant | Acción |
| :--- | :--- | :--- |
| `0x01` | `HealSubstrate` | Inicia scrubbing de RAM y VFS. |
| `0x02` | `OptimizeFlow` | Rebalancea el índice de conflicto. |
| `0x03` | `SyncMesh` | Broadcast de ping a la red micelial. |
| `0x99` | `EmergencyWipe` | Incineración de claves de identidad. |