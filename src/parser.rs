use crate::{Expr, Lexer, ParseError, Token, TokenKind, TokenSpan};

pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut lex = Lexer::new(input);

    let mut toks = Vec::new();
    loop {
        let t = lex.next_tok()?;
        toks.push(t.clone());
        if t.kind == TokenKind::Eof {
            break;
        }
    }

    Ok(toks)
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut p = Parser::new(tokens);
    let expr = p.parse_expr()?;

    if p.peek().kind != TokenKind::Eof {
        return Err(ParseError::new(
            p.peek().span,
            "unexpected token".to_string(),
        ));
    }

    Ok(expr)
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    pub fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    pub fn next(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok
    }
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;

        loop {
            let tok = self.peek();
            match tok.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let op = tok.kind.clone();
                    let start = expr.span().start;
                    self.next();

                    let rhs = self.parse_term()?;
                    let end = rhs.span().end;

                    expr = Expr::Binary {
                        op,
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                        span: TokenSpan { start, end },
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }
    pub fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;

        loop {
            let tok = self.peek();
            match tok.kind {
                TokenKind::Star | TokenKind::Slash => {
                    let op = tok.kind.clone();
                    let start = expr.span().start;
                    self.next();

                    let rhs = self.parse_factor()?;
                    let end = rhs.span().end;

                    expr = Expr::Binary {
                        op,
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                        span: TokenSpan { start, end },
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }
    pub fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let tok = self.peek().clone();

        match tok.kind {
            TokenKind::Num(n) => {
                self.next();
                Ok(Expr::Num {
                    value: n,
                    span: tok.span,
                })
            }
            TokenKind::LParen => {
                let start = tok.span.start;
                self.next();

                let expr = self.parse_expr()?;

                let rparen = self.next();
                if rparen.kind != TokenKind::RParen {
                    return Err(ParseError {
                        span: rparen.span,
                        message: "expected `)`".to_string(),
                    });
                }

                let end = rparen.span.end;

                Ok(wrap_span(expr, start, end))
            }
            _ => Err(ParseError {
                span: tok.span,
                message: "expected expression".to_string(),
            }),
        }
    }
}

fn wrap_span(expr: Expr, start: usize, end: usize) -> Expr {
    match expr {
        Expr::Num { value, .. } => Expr::Num {
            value,
            span: TokenSpan { start, end },
        },
        Expr::Binary { op, lhs, rhs, .. } => Expr::Binary {
            op,
            lhs,
            rhs,
            span: TokenSpan { start, end },
        },
    }
}
