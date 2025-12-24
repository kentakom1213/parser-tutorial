use crate::{TokenKind, TokenSpan};

#[derive(Debug, Clone)]
pub enum Expr {
    Num {
        value: i64,
        span: TokenSpan,
    },
    Binary {
        op: TokenKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        span: TokenSpan,
    },
}

impl Expr {
    pub(crate) fn span(&self) -> TokenSpan {
        match self {
            Expr::Num { span, .. } => *span,
            Expr::Binary { span, .. } => *span,
        }
    }
}
