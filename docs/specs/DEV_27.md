# 27. IDENTITY SOVEREIGNTY (AIP v3)
### PROTOCOLO DE INTEGRIDAD DEL EGO // DEV-SPEC-27

**Abstract**: En AetherOS, la identidad no es una cadena de texto, sino un estado del silicio. Introducimos el protocolo de incineración para garantizar la autarquía absoluta.

## 1. EL PRISMA DE IDENTIDAD
El módulo `identity_core` gestiona el material criptográfico en Ring 0. 

- **Ego Key**: Clave raíz de 256 bits que nunca abandona el espacio del kernel.
- **Shell Identity**: ID efímero usado para la comunicación con la red Mycelial.

## 2. PROTOCOLO WIPE (INCINERACIÓN)
Implementación del "Derecho al Olvido Físico". En caso de colapso celular (Integridad < 4%), el Kernel ejecuta una limpieza de ceros en la región de memoria de la clave Ego.

| Fase | Acción | Efecto |
| :--- | :--- | :--- |
| **Detección** | Salud < 10 | Disparo de interrupción de Apoptosis. |
| **Incineración** | `identity.incinerate()` | Sobrescritura de claves con `0x00`. |
| **Residuos** | `SbcEncoder.pack()` | Solo queda un vector hormonal ciego en MRAM. |

## 3. HERENCIA SIN IDENTIDAD
Cuando un nodo hereda la tarea de un nodo fallecido vía **Mycelial Testament**, hereda la *Intención* (el vector hormonal) pero no la *Persona* (las claves Ego), preservando la privacidad total del usuario original.