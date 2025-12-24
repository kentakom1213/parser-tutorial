use crate::{Lexer, Token, TokenKind};

pub fn parse(input: &str) -> Result<Vec<Token>, String> {
    let mut lex = Lexer::new(input);

    let mut toks = Vec::new();
    loop {
        let t = lex
            .next_tok()
            .map_err(|e| format!("err@{:?}: {}", e.span, e.message))?;
        toks.push(t.clone());
        if t.kind == TokenKind::Eof {
            break;
        }
    }

    Ok(toks)
}
