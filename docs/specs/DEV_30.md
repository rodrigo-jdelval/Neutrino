# 30. SYNAPTIC FRACTALIZATION
### PROTOCOLO DE DESCOMPOSICIÓN HYDRA // DEV-SPEC-30

**Abstract**: Documentación del mecanismo de sharding de intenciones. Un nodo Omega no ejecuta tareas, las fractaliza para el enjambre.

## 1. LA NECESIDAD DE FRACTALIZAR
Una intención compleja (ej. "Entrenar modelo local") excede la capacidad de un solo nodo. El modulo `fractal_core` permite dividir el grafo de dependencia en fragmentos ejecutables independientemente.

## 2. EL SHARD (FRAGMENTO)
Un Shard contiene:
- **Parent ID**: Vínculo con la intención original.
- **Complexity**: Estimación de Joules/Tokens requeridos.
- **Distributed Bit**: Flag que indica si puede ser subastado en la red Hydra.

## 3. SUBASTA DE SHARDS
Los shards con el flag `is_distributed` activo son publicados en la red micelial. Los nodos vecinos pujan por ellos basándose en su propia salud allostática (Melatonina baja, Adrenalina alta).