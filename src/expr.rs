use crate::token::Token;
use std::fmt::{Display, Formatter};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub enum LiteralValue {
    Bool(bool),
    String(String),
    Number(f64),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", operator.token_type, left, right),
            Expr::Grouping { expression } => write!(f, "(group {})", expression),
            Expr::Literal { value } => write!(f, "{}", value),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator.lexeme, right),
        }
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Bool(value) => write!(f, "{}", value),
            LiteralValue::String(value) => write!(f, "\"{}\"", value),
            LiteralValue::Number(value) => write!(f, "{}", value),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}
