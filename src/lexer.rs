use std::{iter::Peekable, str::CharIndices};

use crate::{ParseError, Token, TokenKind, TokenSpan};

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
    fn bump_with_span(&mut self) -> TokenSpan {
        let start = self.pos();
        self.bump();
        let end = self.pos();
        TokenSpan { start, end }
    }
    pub fn next_tok(&mut self) -> Result<Token, ParseError> {
        // 空白をスキップ
        self.skip_ws();

        let pos = self.pos();
        let Some(b) = self.peek() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: TokenSpan {
                    start: pos,
                    end: pos,
                },
            });
        };

        match b {
            '+' => {
                let span = self.bump_with_span();
                Ok(Token {
                    kind: TokenKind::Plus,
                    span,
                })
            }
            '-' => {
                let span = self.bump_with_span();
                Ok(Token {
                    kind: TokenKind::Minus,
                    span,
                })
            }
            '*' => {
                let span = self.bump_with_span();
                Ok(Token {
                    kind: TokenKind::Star,
                    span,
                })
            }
            '/' => {
                let span = self.bump_with_span();
                Ok(Token {
                    kind: TokenKind::Slash,
                    span,
                })
            }
            '(' => {
                let span = self.bump_with_span();
                Ok(Token {
                    kind: TokenKind::LParen,
                    span,
                })
            }
            ')' => {
                let span = self.bump_with_span();
                Ok(Token {
                    kind: TokenKind::RParen,
                    span,
                })
            }
            '0'..='9' => {
                let start = self.pos();
                let mut n = 0;
                while let Some(c @ '0'..='9') = self.peek() {
                    self.bump();
                    n = n * 10 + (c as i64 - '0' as i64);
                }
                let end = self.pos();
                Ok(Token {
                    kind: TokenKind::Num(n),
                    span: TokenSpan { start, end },
                })
            }
            _ => Err(ParseError::new(
                TokenSpan {
                    start: pos,
                    end: pos,
                },
                "unexpected char",
            )),
        }
    }
}
