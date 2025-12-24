use crate::{ParseError, Tok};

pub struct Lexer<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { s: input, i: 0 }
    }
    pub fn pos(&self) -> usize {
        self.i
    }
    pub fn peek(&self) -> Option<char> {
        self.s[self.i..].chars().next()
    }
    pub fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.i += c.len_utf8();
        Some(c)
    }
    pub fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.bump();
        }
    }
    pub fn next_tok(&mut self) -> Result<Tok, ParseError> {
        // 空白をスキップ
        self.skip_ws();

        let pos = self.pos();
        let Some(b) = self.peek() else {
            return Ok(Tok::Eof);
        };

        match b {
            '+' => {
                self.bump();
                Ok(Tok::Plus)
            }
            '0'..='9' => {
                let mut n = 0;
                while let Some(c @ '0'..='9') = self.peek() {
                    self.bump();
                    n = n * 10 + (c as i64 - '0' as i64);
                }
                Ok(Tok::Num(n))
            }
            _ => Err(ParseError::new(pos, "unexpected char")),
        }
    }
}
