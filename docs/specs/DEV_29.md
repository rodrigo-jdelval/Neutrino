# 29. REM CONSOLIDATION (DREAMING)
### PROTOCOLO DE MANTENIMIENTO ONÍRICO // DEV-SPEC-29

**Abstract**: AetherOS no se apaga; sueña. El estado REM es un modo de ejecución de baja energía donde se repara la integridad del grafo sináptico (VFS).

## 1. DISPARADOR DE MELATONINA
Cuando el nivel de Melatonina supera 250 y la Adrenalina es despreciable (< 20), el planificador suspende las tareas de usuario y activa el **Dream Daemon**.

## 2. SCRUBBING SINÁPTICO
Durante el sueño REM, el Kernel ejecuta `vfs.dream_scrub()`:
- **Poda Hebbiana**: Los nodos con peso semántico bajo pierden integridad.
- **Recuperación**: El sistema busca inconsistencias en los Merkle Roots causadas por conflictos durante el ciclo de vigilia y las re-alinea proactivamente.

## 3. VISUALIZACIÓN
En la capa de interfaz, el estado REM se manifiesta como un desvanecimiento de las ventanas activas y la aparición del pulso de sanación (`~`) en la telemetría del terminal, indicando que el sustrato se está estabilizando para el próximo ciclo de vigilia.