// libaether: THE ORGANISM LIBRARY v3.0 (Hydra Mesh Aware)
// Abstracciones de alto nivel para aplicaciones soberanas.

pub mod proprio {
    /// Devuelve la integridad percibida del sustrato (0-255)
    pub fn get_cellular_integrity() -> u32 {
        unsafe { peekb(0x38210) as u32 }
    }

    /// Devuelve el índice de conflicto actual (Nash Equilibrium status)
    pub fn get_conflict_index() -> u32 {
        unsafe { peekb(0x38260) as u32 }
    }
}

pub mod bio {
    /// Secreta Adrenalina para ganar prioridad táctica (Panic Button)
    pub fn panic_button() {
        unsafe { syscall(200, 0, 255, 0, 0); }
    }

    /// Secreta Melatonina para ceder ciclos de CPU y permitir sanación
    pub fn meditate() {
        unsafe { syscall(200, 2, 255, 0, 0); }
    }

    /// Lee el nivel actual de Cortisol (Estrés)
    pub fn get_stress() -> u32 {
        unsafe { peekb(0x38204) as u32 }
    }
}

pub mod drivers {
    pub fn rng() -> u32 {
        unsafe { peek(0x38300) as u32 }
    }
    
    pub fn uptime() -> u32 {
        unsafe { peek(0x38304) as u32 }
    }
}

pub mod npu {
    // FAST-PATH BINARY OPS (Phase 1)
    pub enum ReflexOp {
        HealSubstrate = 0x01,
        OptimizeFlow  = 0x02,
        SyncMesh      = 0x03,
        EmergencyWipe = 0x99
    }

    /// Ejecuta una acción refleja instantánea (Fast-Path)
    /// Bypass total del motor de inferencia. <1ms de latencia.
    pub fn reflex(op: ReflexOp) {
        unsafe { syscall(250, 0x200, op as usize, 0, 0); }
    }

    /// Envía una intención al motor de inferencia de frontera
    pub fn execute_intent(intent_ptr: usize, len: usize) -> usize {
        unsafe { syscall(250, 0x101, intent_ptr, len, 0) }
    }
}

pub mod fs {
    /// Abre un archivo y devuelve un descriptor (FD).
    /// Retorna -1 si falla.
    pub fn open(path_ptr: usize) -> usize {
        unsafe { syscall(5, path_ptr, 0, 0, 0) }
    }

    /// Escribe un buffer en un descriptor de archivo.
    pub fn write(fd: usize, buf_ptr: usize, len: usize) -> usize {
        unsafe { syscall(4, fd, buf_ptr, len, 0) }
    }

    /// Lee de un descriptor de archivo a un buffer.
    pub fn read(fd: usize, buf_ptr: usize, len: usize) -> usize {
        unsafe { syscall(3, fd, buf_ptr, len, 0) }
    }

    /// Cierra un descriptor de archivo.
    pub fn close(fd: usize) -> usize {
        unsafe { syscall(6, fd, 0, 0, 0) }
    }
}

pub mod mesh {
    /// Transmite un vector semántico (mensaje) a la red micelial local.
    /// Utiliza el protocolo Hydra Lite (BroadcastChannel).
    pub fn broadcast(vector_ptr: usize, len: usize) {
        // Syscall 300: Hydra Broadcast
        unsafe { syscall(300, vector_ptr, len, 0, 0); }
    }

    /// Descubre nodos vecinos y retorna el conteo.
    /// Los detalles se escriben en el buffer del sistema (simulado).
    pub fn scan() -> u32 {
        // Syscall 301: Hydra Scan
        unsafe { syscall(301, 0, 0, 0, 0) }
    }
}

pub mod crypto {
    /// Genera un hash SHA-256 del contenido (Offloaded al Host).
    pub fn sha256(data_ptr: usize, len: usize, out_ptr: usize) {
        // Syscall 250 (Offload) con Opcode 0x01 (HASH)
        unsafe { syscall(250, 0x01, data_ptr, len, out_ptr); }
    }
}