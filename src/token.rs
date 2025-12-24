#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Num(i64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TokenSpan,
}
