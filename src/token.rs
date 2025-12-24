#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Num(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}
