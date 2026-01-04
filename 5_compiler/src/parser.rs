fn peek_tok_type() { 
    let idx = 0; unsafe { idx = peek(G_TOK_IDX); }
    let count = 0; unsafe { count = peek(G_TOK_COUNT); }
    if idx >= count { return 0; }
    let t = 0; unsafe { t = peek(TOKEN_BUF + (idx*8)); } 
    return t;
}
fn peek_tok_val() { 
    let idx = 0; unsafe { idx = peek(G_TOK_IDX); }
    let v = 0; unsafe { v = peek(TOKEN_BUF + (idx*8) + 4); } 
    return v;
}
fn consume() { let idx = 0; unsafe { idx = peek(G_TOK_IDX); poke(G_TOK_IDX, idx + 1); } }

fn alloc_reg() { let r=0; unsafe{r=peek(G_NEXT_REG); poke(G_NEXT_REG, r+1);} return r; }
fn map_var(hash, reg) { let idx=reg-18; unsafe{poke(SYM_TABLE+(idx*8), hash); poke(SYM_TABLE+(idx*8)+4, reg);} }
fn find_var(hash) {
    let max=0; unsafe{max=peek(G_NEXT_REG);} let r=18;
    while r < max {
        let h=0; unsafe{h=peek(SYM_TABLE+((r-18)*8));}
        if h==hash { return r; }
        r=r+1;
    }
    return 0;
}

fn register_func(hash, addr) {
    let cnt = 0; unsafe { cnt = peek(G_FUNC_COUNT); }
    unsafe { poke(FUNC_TABLE + (cnt*8), hash); poke(FUNC_TABLE + (cnt*8)+4, addr); poke(G_FUNC_COUNT, cnt+1); }
}
fn find_func_addr(hash) {
    let cnt = 0; unsafe { cnt = peek(G_FUNC_COUNT); }
    let i = 0;
    while i < cnt {
        let h = 0; unsafe { h = peek(FUNC_TABLE + (i*8)); }
        if h == hash { let a = 0; unsafe { a = peek(FUNC_TABLE + (i*8)+4); } return a; }
        i = i + 1;
    }
    return 0;
}
fn add_patch(hash, addr) {
    let cnt = 0; unsafe { cnt = peek(G_PATCH_COUNT); }
    unsafe { poke(PATCH_TABLE + (cnt*8), hash); poke(PATCH_TABLE + (cnt*8)+4, addr); poke(G_PATCH_COUNT, cnt+1); }
}

fn parse_term(target_reg) {
    let t = peek_tok_type();
    let v = peek_tok_val();
    consume(); 
    
    if t == T_NUM { emit_addi(target_reg, 0, v); } 
    else if t == T_ID {
        let next = peek_tok_type();
        if next == T_LPAREN || next == T_DCOLON {
            parse_call(v); 
            emit_addi(target_reg, 10, 0); 
        } else {
            let src = find_var(v);
            if src > 0 { emit_addi(target_reg, src, 0); } 
            else { emit_addi(target_reg, 0, 0); }
        }
    }
}

fn parse_expr(target_reg) {
    parse_term(target_reg);
    loop {
        let t = peek_tok_type();
        if t==43||t==45||t==42||t==38||t==124||t==94||t==201||t==202 { 
            consume();
            let r2 = alloc_reg();
            parse_term(r2);
            emit_op(target_reg, target_reg, r2, t);
        } else { break; }
    }
}

fn parse_call(hash_in) {
    let mut final_hash = hash_in;
    if peek_tok_type() == T_DCOLON {
        consume(); 
        let method_hash = peek_tok_val(); consume();
        final_hash = hash_in ^ method_hash;
    }
    consume(); 
    let mut arg_idx = 10; 
    while peek_tok_type() != T_RPAREN {
        parse_expr(arg_idx); 
        arg_idx = arg_idx + 1;
        if peek_tok_type() == T_COMMA { consume(); }
    }
    consume(); 
    
    let addr = find_func_addr(final_hash);
    if addr > 0 { 
        let pc = get_code_pc();
        emit_jal(1, addr - pc); 
    } else {
        let pc = get_code_pc();
        add_patch(final_hash, pc);
        emit_jal(1, 0); 
    }
}

fn parse_stmt() {
    let t = peek_tok_type();
    if t == T_LBRACE { consume(); while peek_tok_type() != T_RBRACE { parse_stmt(); } consume(); return; }
    if t == T_IF { parse_if(); return; }
    if t == T_WHILE { parse_while(); return; }
    if t == T_LOOP { parse_loop(); return; }
    if t == T_UNSAFE { consume(); parse_stmt(); return; } 
    
    if t == T_LET {
        consume(); 
        if peek_tok_type() == 325 { consume(); } 
        let name = peek_tok_val(); consume(); 
        consume(); 
        let reg = alloc_reg();
        map_var(name, reg);
        parse_expr(reg);
        consume(); 
        return;
    }
    
    if t == T_ID {
        let name = peek_tok_val();
        let next = 0; unsafe { next = peek(TOKEN_BUF + ((get_tok_idx()+1)*8)); }
        if next == T_EQ {
            consume(); consume(); 
            let reg = find_var(name);
            if reg > 0 { parse_expr(reg); } else { let temp = alloc_reg(); parse_expr(temp); }
            consume(); 
            return;
        }
    }
    
    let temp = alloc_reg();
    parse_expr(temp);
    if peek_tok_type() == T_SEMI { consume(); }
}

fn get_tok_idx() { let i=0; unsafe{i=peek(G_TOK_IDX);} return i; }

fn compile_program() {
    unsafe { poke(G_CODE_IDX, 84); } 
    add_patch(425, 84); 
    emit_jal(0, 0);
    
    while 1 {
        let t = peek_tok_type();
        if t == 0 { break; }
        if t == T_STRUCT { parse_struct(); }
        else if t == T_IMPL { parse_impl(); }
        else if t == T_FN { parse_fn(0); }
        else if t == T_CONST { consume(); consume(); consume(); consume(); consume(); consume(); }
        else { consume(); }
    }
    
    let pcnt = 0; unsafe { pcnt = peek(G_PATCH_COUNT); }
    let k = 0;
    while k < pcnt {
        let h=0; let a=0;
        unsafe{ h=peek(PATCH_TABLE+(k*8)); a=peek(PATCH_TABLE+(k*8)+4); }
        let tgt = find_func_addr(h);
        if tgt > 0 { patch_jump(a, tgt - a); }
        k = k + 1;
    }
}