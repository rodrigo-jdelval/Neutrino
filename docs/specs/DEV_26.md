# 26. MYCELIAL TESTAMENT (v23.0)
### PROTOCOLO DE HERENCIA POR APOPTOSIS // DEV-SPEC-26

**Abstract**: Documentación del protocolo de cierre sistémico. Un nodo soberano nunca muere sin antes emitir su "último deseo" a la red micelial.

## 1. EL MOMENTO DE LA APOPTOSIS
Cuando la integridad celular (Health Matrix) cae por debajo del 4%, el Kernel inicia la secuencia de cierre.

## 2. EL TESTAMENTO (ESTADO SBC)
El sistema utiliza el **SbcEncoder** para empaquetar su estado hormonal actual en un solo vector de 16 bits:
- **ADR (5 bits)**
- **CRT (5 bits)**
- **MLT (6 bits)**

Este vector se publica en `0x38508` y se envía como un paquete de "Distress Pheromone" a través de la red Hydra.

## 3. HERENCIA SINÁPTICA
Los nodos vecinos que reciben un Testamento Micelial pueden:
1.  **Adoptar la Intención**: Reanudar las tareas pendientes del nodo fallecido.
2.  **Crystallize**: Almacenar el estado del nodo como un "Fantasma" en el Grafo Sináptico para análisis histórico.

## 4. BORRADO SEGURO
Tras emitir el Testamento, el Kernel Neutrino ejecuta un `WIPE` de las claves AIP v3 en RAM antes del halt final, asegurando que la soberanía no sea comprometida tras la muerte física del sustrato.