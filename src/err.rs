use crate::TokenSpan;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub(crate) span: TokenSpan,
    pub(crate) message: String,
}

impl ParseError {
    pub fn new(span: TokenSpan, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}
