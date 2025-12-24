use crate::{Lexer, Tok};

pub fn parse(input: &str) -> Result<Vec<Tok>, String> {
    let mut lex = Lexer::new(input);

    let mut toks = Vec::new();
    loop {
        let t = lex
            .next_tok()
            .map_err(|e| format!("err@{}: {}", e.pos, e.message))?;
        toks.push(t.clone());
        if t == Tok::Eof {
            break;
        }
    }

    Ok(toks)
}
