use crate::token_type::TokenType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub count: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, count: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            count,
        }
    }
}
