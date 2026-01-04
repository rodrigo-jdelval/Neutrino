fn is_digit(c) { if c >= 48 { if c <= 57 { return 1; } } return 0; }
fn is_alpha(c) { 
    if c >= 97 { if c <= 122 { return 1; } } 
    if c >= 65 { if c <= 90 { return 1; } }
    if c == 95 { return 1; } 
    return 0; 
}
fn is_space(c) { if c == 32 { return 1; } if c == 10 { return 1; } if c == 13 { return 1; } if c == 9 { return 1; } return 0; }

fn tokenize(src_addr, len) {
    let i = 0;
    let tok_idx = 0;
    unsafe { poke(G_TOK_COUNT, 0); }

    while i < len {
        let c = 0; unsafe { c = peekb(src_addr + i); }

        if is_space(c) == 1 { i = i + 1; continue; }

        if c == 47 { 
            let next = 0; unsafe { next = peekb(src_addr + i + 1); }
            if next == 47 {
                while i < len {
                    let nc = 0; unsafe { nc = peekb(src_addr + i); }
                    if nc == 10 { break; }
                    i = i + 1;
                }
                continue;
            }
        }

        if c == 58 { 
            let next = 0; unsafe { next = peekb(src_addr + i + 1); }
            if next == 58 { 
                unsafe { poke(TOKEN_BUF + (tok_idx*8), T_DCOLON); poke(TOKEN_BUF + (tok_idx*8)+4, 0); }
                tok_idx = tok_idx + 1; i = i + 2; continue;
            }
        }
        if c == 45 { 
            let next = 0; unsafe { next = peekb(src_addr + i + 1); }
            if next == 62 { 
                unsafe { poke(TOKEN_BUF + (tok_idx*8), T_ARROW); poke(TOKEN_BUF + (tok_idx*8)+4, 0); }
                tok_idx = tok_idx + 1; i = i + 2; continue;
            }
        }

        if c==123||c==125||c==40||c==41||c==59||c==44||c==61||c==43||c==45||c==42||c==46||c==58 {
            unsafe { poke(TOKEN_BUF + (tok_idx*8), c); poke(TOKEN_BUF + (tok_idx*8)+4, 0); }
            tok_idx = tok_idx + 1; i = i + 1; continue;
        }

        if is_digit(c) == 1 {
            let val = 0;
            while i < len {
                let d = 0; unsafe { d = peekb(src_addr + i); }
                if is_digit(d) == 0 { break; }
                val = (val * 10) + (d - 48);
                i = i + 1;
            }
            unsafe { poke(TOKEN_BUF + (tok_idx*8), T_NUM); poke(TOKEN_BUF + (tok_idx*8)+4, val); }
            tok_idx = tok_idx + 1; continue;
        }

        if is_alpha(c) == 1 {
            let hash = 0;
            while i < len {
                let char_code = 0; unsafe { char_code = peekb(src_addr + i); }
                if is_alpha(char_code) == 0 { if is_digit(char_code) == 0 { break; } }
                hash = ((hash << 5) + hash) + char_code;
                i = i + 1;
            }
            
            let type = T_ID;
            // Simple hash matching for keywords
            if hash == 5863486 { type = T_FN; }       // fn
            if hash == 193496674 { type = T_LET; }    // let
            if hash == 6952303156 { type = T_STRUCT; }// struct
            if hash == 254746356 { type = T_CONST; }  // const
            if hash == 2090487428 { type = T_IMPL; }  // impl
            if hash == 193503681 { type = T_PUB; }    // pub
            if hash == 2139462529 { type = T_RETURN; }// return
            if hash == 229342186 { type = T_UNSAFE; } // unsafe
            if hash == 2107083626 { type = T_WHILE; } // while
            if hash == 2090422997 { type = T_LOOP; }  // loop
            if hash == 5863371 { type = T_IF; }       // if
            if hash == 2090250669 { type = T_ELSE; }  // else

            unsafe { poke(TOKEN_BUF + (tok_idx*8), type); poke(TOKEN_BUF + (tok_idx*8)+4, hash); }
            tok_idx = tok_idx + 1; continue;
        }
        i = i + 1;
    }
    unsafe { poke(G_TOK_COUNT, tok_idx); }
}