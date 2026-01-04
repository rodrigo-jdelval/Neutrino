# 25. HYDRA LITE: LOCAL MESH PROTOCOL
### ESPECIFICACIÓN DE RED // DEV-SPEC-25

**Abstract**: Hydra Lite es la implementación de la Fase 3 para descubrimiento y consenso en entornos de navegador. Permite que múltiples instancias de AetherOS (pestañas/ventanas) formen un superordenador virtual.

## 1. CAPA DE TRANSPORTE
Utiliza la API `BroadcastChannel` del navegador como sustrato físico simulado.
- **Canal**: `AETHER_MYCELIUM_V1`
- **Topología**: Bus compartido (Ethernet simulado). Todos escuchan a todos.

## 2. ESTRUCTURA DEL PAQUETE (FEROMONA)
```typescript
interface Pheromone {
    type: 'HEARTBEAT' | 'PING' | 'INTENT_GOSSIP';
    senderId: string; // NODE_XXXX
    payload: {
        status: 'IDLE' | 'BUSY';
        trustScore: number;
        hormones: { adr: u8, crt: u8, mlt: u8 };
    };
    timestamp: number;
}
```

## 3. DINÁMICA DE CONFIANZA
El Kernel mantiene una tabla de enrutamiento (`ModuleHydra`).
- **Refuerzo**: Recibir un `HEARTBEAT` válido incrementa la confianza (+0.05).
- **Apoptosis**: Si un nodo no emite señal en 15s, se considera "Muerto" y se elimina de la tabla para liberar memoria.

## 4. USO DESDE RUST (SDK)
```rust
use aether::mesh;

fn main() {
    // Escanear vecinos
    let peers = mesh::scan();
    
    if peers > 0 {
        // Enviar vector de intención a la red
        mesh::broadcast("SYNC_REQ");
    }
}
```