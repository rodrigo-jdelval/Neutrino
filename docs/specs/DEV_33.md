# 33. QUIESCENCE & METABOLIC GATING
### PROTOCOLO DE SUPERVIVENCIA ENERGÉTICA // DEV-SPEC-33

**Abstract**: Este documento define la relación entre la solvencia de tokens y la capacidad física de ejecución del CPU virtual RISC-V.

## 1. GATING METABÓLICO
En AetherOS, el "derecho a computar" no es infinito. El planificador implementa un gate (puerta) lógico en Ring 0:
- **Solvencia**: Si `token_pool > 0`, el kernel permite el paso de instrucciones al intérprete.
- **Bancarrota**: Si `token_pool <= 0`, el kernel emite una señal de interrupción interna que detiene el motor de inferencia. El nodo entra en estado de "Escucha Pasiva".

## 2. QUIESCENCIA PROFUNDA (WFI)
Inspirado en la instrucción `Wait For Interrupt` de los procesadores físicos, la Quiescencia en AetherOS sincroniza el bucle de JavaScript con el estado hormonal:
- **Trigger**: Melatonina > 240.
- **Acción**: El kernel aumenta el tiempo de `sys_yield()`, permitiendo que el navegador anfitrión reduzca el consumo de CPU y limpie el recolector de basura (GC).

## 3. RECUPERACIÓN DE ESTÍMULO
Un nodo en Quiescencia solo despierta si el **Mycelial Link** detecta una "Inyección de Intención" (User Input) que reinyecta Adrenalina en el bus hormonal.