fn emit(val) { let idx = 0; unsafe { idx = peek(G_CODE_IDX); poke(CODE_BUF + idx, val); poke(G_CODE_IDX, idx + 4); } }
fn emit_byte(b) { let idx = 0; unsafe { idx = peek(G_CODE_IDX); pokeb(CODE_BUF + idx, b); poke(G_CODE_IDX, idx + 1); } }
fn get_target() { let t=0; unsafe{t=peek(G_TARGET);} return t; }
fn get_code_pc() { let idx=0; unsafe{idx=peek(G_CODE_IDX);} return idx; }

fn emit_leb128(val) {
    let mut v = val;
    loop {
        let byte = v & 127;
        v = v >> 7;
        if v == 0 { emit_byte(byte); break; } else { emit_byte(byte | 128); }
    }
}

fn emit_wasm_header() {
    emit_byte(0x00); emit_byte(0x61); emit_byte(0x73); emit_byte(0x6D);
    emit_byte(0x01); emit_byte(0x00); emit_byte(0x00); emit_byte(0x00);
    emit_byte(1); emit_byte(4); emit_byte(1); emit_byte(0x60); emit_byte(0); emit_byte(0); 
    emit_byte(3); emit_byte(2); emit_byte(1); emit_byte(0); 
    emit_byte(7); emit_byte(17); emit_byte(2);  
    emit_byte(6); let mut mem="memory"; let mut i=0; while i<6 { emit_byte(unsafe{peekb(mem+i)}); i=i+1; }
    emit_byte(2); emit_byte(0); 
    emit_byte(6); let mut st="_start"; i=0; while i<6 { emit_byte(unsafe{peekb(st+i)}); i=i+1; }
    emit_byte(0); emit_byte(0); 
    emit_byte(10); unsafe { poke(G_PATCH_COUNT, get_code_pc()); } 
    emit_byte(0x80); emit_byte(0x80); emit_byte(0x80); emit_byte(0x00);
    emit_byte(1); 
    unsafe { poke(G_PATCH_COUNT + 4, get_code_pc()); }
    emit_byte(0x80); emit_byte(0x80); emit_byte(0x80); emit_byte(0x00);
    emit_byte(0); 
}

fn emit_wasm_footer() { emit_byte(0x0B); }

fn emit_op(rd, rs1, rs2, op_token) {
    if get_target() == 0 {
        let funct3 = 0; let funct7 = 0;
        if op_token == 43 { funct3 = 0; funct7 = 0; }    
        if op_token == 45 { funct3 = 0; funct7 = 32; }   
        if op_token == 38 { funct3 = 7; funct7 = 0; }    
        if op_token == 124 { funct3 = 6; funct7 = 0; }   
        let bin = (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x33;
        emit(bin);
    }
}

fn emit_addi(rd, rs1, imm) {
    if get_target() == 0 {
        let bin = ((imm & 0xFFF) << 20) | (rs1 << 15) | (0 << 12) | (rd << 7) | 0x13;
        emit(bin);
    }
}

fn emit_sw(rs2, rs1, offset) {
    if get_target() == 0 {
        let imm_low = offset & 0x1F;
        let imm_high = (offset >> 5) & 0x7F;
        let bin = (imm_high << 25) | (rs2 << 20) | (rs1 << 15) | (2 << 12) | (imm_low << 7) | 0x23;
        emit(bin);
    }
}

fn emit_lw(rd, rs1, offset) {
    if get_target() == 0 {
        let bin = ((offset & 0xFFF) << 20) | (rs1 << 15) | (2 << 12) | (rd << 7) | 0x03;
        emit(bin);
    }
}

fn emit_jal(rd, offset) {
    if get_target() == 0 {
        let imm20 = (offset >> 20) & 1;
        let imm10_1 = (offset >> 1) & 0x3FF;
        let imm11 = (offset >> 11) & 1;
        let imm19_12 = (offset >> 12) & 0xFF;
        let bin = (imm20 << 31) | (imm10_1 << 21) | (imm11 << 20) | (imm19_12 << 12) | (rd << 7) | 0x6F;
        emit(bin);
    }
}

fn emit_ret() {
    if get_target() == 0 {
        emit(0x00008067); // JALR x0, 0(ra)
    }
}

fn patch_jump(addr, offset) {
    if get_target() == 0 {
        // RISC-V Branch Patching Logic (Simplified)
    }
}