# 35. SUBSTRATE SOVEREIGNTY INVARIANTS
### GARANTÍAS DE AUTARQUÍA // DEV-SPEC-35

**Abstract**: Reglas invariantes que aseguran que AetherOS permanezca soberano frente a la evolución del hardware.

## 1. INDEPENDENCIA DEL ANFITRIÓN
El núcleo Neutrino debe ser capaz de funcionar con solo 7 syscalls básicas. Cualquier capacidad extra (WebGPU, Red) se trata como un "Órgano Opcional".

## 2. INVARIANTE DE NANORUST
El código fuente en `/home/src/neutrino` debe ser siempre compilable por el binario `compiler.elf` contenido en el Seed. Se prohíbe el uso de librerías externas que no puedan ser reducidas a código máquina RV32I.

## 3. RESONANCIA DE DATOS
Un nodo AetherOS nunca debe aceptar datos cuya integridad Merkle no pueda ser verificada localmente. La confianza no se delega; se calcula.