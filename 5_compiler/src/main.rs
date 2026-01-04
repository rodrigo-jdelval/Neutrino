// NanoRust Compiler Entry Point (Stage 11: Multi-Target Sovereignty)
include! "../../0_lib/offload_core/src/lib.rs";

const HEAP_START: usize = 0x200000; 
const TOKEN_BUF: usize  = 0x300000; 
const CODE_BUF: usize   = 0x400000; 
const SYM_TABLE: usize  = 0x201000; 

const G_TOK_COUNT: usize = 0x1F0000;
const G_TOK_IDX: usize   = 0x1F0004;
const G_CODE_IDX: usize  = 0x1F0008;
const G_NEXT_REG: usize  = 0x1F010;
const G_FUNC_COUNT: usize = 0x1F0014;
const G_PATCH_COUNT: usize = 0x1F0018;
const G_TARGET: usize    = 0x1F0030; // 0=RISCV, 1=WASM

const T_FN: usize=1; const T_LET: usize=2; const T_CONST: usize=3; 
const T_STRUCT: usize=4; const T_ID: usize=5; const T_NUM: usize=6; 
const T_IMPL: usize=7; const T_PUB: usize=8; const T_RETURN: usize=9; 
const T_UNSAFE: usize=10; const T_WHILE: usize=11; const T_IF: usize=12; 
const T_ELSE: usize=13; const T_LOOP: usize=14;
const T_LBRACE: usize=123; const T_RBRACE: usize=125; const T_LPAREN: usize=40; 
const T_RPAREN: usize=41; const T_SEMI: usize=59; const T_COMMA: usize=44; 
const T_EQ: usize=61; const T_DCOLON: usize=201; const T_ARROW: usize=202;

fn sys_write(fd, buf, len) { return syscall(4, fd, buf, len, 0); }
fn sys_read(fd, buf, len) { return syscall(3, fd, buf, len, 0); }
fn sys_open(path, flags) { return syscall(5, path, flags, 0, 0); }
fn sys_close(fd) { return syscall(6, fd, 0, 0, 0); }

fn print(s) {
    let len = 0;
    loop {
        let c = 0; unsafe { c = peekb(s + len); }
        if c == 0 { break; }
        len = len + 1;
    }
    sys_write(1, s, len);
}
fn println(s) { print(s); sys_write(1, "\n", 1); }

include! "codegen.rs";
include! "lexer.rs";
include! "parser.rs";

fn main() {
    println("[NANORUST] Sovereign Compiler v11.0 (Dual-Target)");
    
    unsafe { 
        poke(G_CODE_IDX, 0); 
        poke(G_TOK_IDX, 0);
        poke(G_FUNC_COUNT, 0);
        poke(G_PATCH_COUNT, 0);
        poke(G_TARGET, 0); // Default to RISC-V
    }

    let src_path = "/home/src/profiles/civilian/main.rs"; 
    let out_path = "/home/civilian.elf";
    
    let fd = sys_open(src_path, 0);
    if fd < 0 { println("[ERR] Source not found."); return; }
    let read_len = sys_read(fd, HEAP_START, 131072);
    sys_close(fd);
    
    if read_len <= 0 { println("[ERR] Empty source."); return; }
    
    tokenize(HEAP_START, read_len);

    let target = 0; unsafe { target = peek(G_TARGET); }
    if target == 0 {
        unsafe { poke(G_CODE_IDX, 84); }
        add_patch(425, 84); 
        emit_jal(0, 0);
    } else {
        emit_wasm_header();
    }

    compile_program();

    let code_size = 0; unsafe { code_size = peek(G_CODE_IDX); }
    
    if target == 0 {
        emit_elf_header(code_size - 84);
        resolve_patches(); 
        println("[OK] Crystallized RISC-V ELF.");
    } else {
        emit_wasm_footer();
        println("[OK] Crystallized WASM Module.");
    }

    let out_fd = sys_open(out_path, 577); 
    sys_write(out_fd, CODE_BUF, code_size);
    sys_close(out_fd);
}