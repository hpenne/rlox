use crate::token_type::TokenType;
use std::fmt::{Display, Formatter};

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type.to_string(), self.lexeme)
    }
}
