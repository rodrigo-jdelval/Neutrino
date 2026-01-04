# 38. THE ORGANISM LIBRARY (libaether)
### ABSTRACCIONES DE ALTO NIVEL // DEV-SPEC-38

**Abstract**: Mientras que el SDK Base define los impulsos eléctricos (Syscalls), `libaether` define el comportamiento del organismo. Proporciona wrappers seguros y ergonómicos para NanoRust.

## 1. MÓDULO DE SENSORES (Interocepción)
Permite consultar el estado fisiológico sin conocer el mapa de memoria GSV.

```rust
use aether::proprio;

fn monitor() {
    let health = proprio::get_cellular_integrity(); // Retorna 0.0 - 1.0
    if health < 0.5 {
        aether::log(LogLevel::Warn, "Substrato degradado detectado.");
    }
}
```

## 2. EL BUS HORMONAL (Secreción Selectiva)
Abstracción para modular el comportamiento del planificador desde la aplicación.

| Función | Propósito | Efecto en el Kernel |
| :--- | :--- | :--- |
| `bio::panic_button()` | Secreta Adrenalina al máximo. | Preemption inmediata, subasta prioritaria. |
| `bio::meditate()` | Secreta Melatonina. | Cede el control por tiempo prolongado (Sleep). |
| `bio::work_flow()` | Mantiene el Cortisol bajo. | Reduce el Nash Tax (impuesto por cambio de contexto). |

## 3. SEMANTIC PIPES (NKP Interface)
Permite enviar intenciones complejas a la NPU de forma estructurada.

```rust
use aether::npu;

fn execute_logic() {
    // Macro que genera el JSON del protocolo NKP
    let result = intent!("Analizar el VFS en busca de nodos huérfanos y sugerir poda.");
    
    match result {
        IntentResult::Success(observation) => println!("Análisis completo: {}", observation),
        IntentResult::Dissonance => panic!("Intento rechazado por el Juez."),
    }
}
```

## 4. GESTIÓN DE RECURSOS (RAII Soberano)
Aunque NanoRust no tiene destructores automáticos complejos, `libaether` implementa el patrón de **Cierre de Ciclo**:

```rust
fn write_protected() {
    let mut file = fs::open("/etc/secure.bin", OpenMode::Hardened);
    file.write_all(data);
    // El método close() es obligatorio para liberar el FD y evitar 'Cognitive Leaks'
    file.close(); 
}
```

## 5. REGLA DE ORO DE libaether
Toda función en esta biblioteca debe ser **No Bloqueante**. Si una operación requiere tiempo (I/O, Inferencia), debe devolver un `ResonancePromise` que el desarrollador debe verificar mediante `poll()`.