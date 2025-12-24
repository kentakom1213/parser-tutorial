use std::{iter::Peekable, str::CharIndices};

use crate::{ParseError, Tok};

pub struct Lexer<'a> {
    s: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            s: input,
            iter: input.char_indices().peekable(),
        }
    }
    pub fn pos(&mut self) -> usize {
        self.iter.peek().map(|(i, _)| *i).unwrap_or(self.s.len())
    }
    pub fn peek(&mut self) -> Option<char> {
        self.iter.peek().map(|(_, c)| *c)
    }
    pub fn bump(&mut self) -> Option<char> {
        self.iter.next().map(|(_, c)| c)
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
            '-' => {
                self.bump();
                Ok(Tok::Minus)
            }
            '*' => {
                self.bump();
                Ok(Tok::Star)
            }
            '/' => {
                self.bump();
                Ok(Tok::Slash)
            }
            '(' => {
                self.bump();
                Ok(Tok::LParen)
            }
            ')' => {
                self.bump();
                Ok(Tok::RParen)
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
