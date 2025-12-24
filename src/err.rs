#[derive(Debug, Clone)]
pub struct ParseError {
    pub(crate) pos: usize,
    pub(crate) message: String,
}

impl ParseError {
    pub fn new(pos: usize, message: impl Into<String>) -> Self {
        Self {
            pos,
            message: message.into(),
        }
    }
}
