use crate::error_reporter;
use crate::token::Token;
use std::fmt::{Display, Formatter};

pub enum Expr {
    Assign {
        name: Token,
        expression: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
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
    Variable {
        name: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Bool(bool),
    String(String),
    Number(f64),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assign { name, expression } => write!(f, "({name} = {expression})"),
            Expr::Binary {
                operator,
                left,
                right,
            }
            | Expr::Logical {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", operator.token_type, left, right),
            Expr::Grouping { expression } => write!(f, "(group {expression})"),
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator.lexeme, right),
            Expr::Variable { name } => write!(f, "{name}"),
        }
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Bool(value) => write!(f, "{value}"),
            LiteralValue::String(value) => write!(f, "{value}"),
            LiteralValue::Number(value) => write!(f, "{value}"),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

impl TryFrom<LiteralValue> for f64 {
    type Error = error_reporter::Error;

    fn try_from(value: LiteralValue) -> Result<Self, Self::Error> {
        if let LiteralValue::Number(number) = value {
            return Ok(number);
        }
        Err(error_reporter::Error {
            token: None,
            message: format!("{value} is not a number"),
        })
    }
}

impl TryFrom<LiteralValue> for String {
    type Error = error_reporter::Error;

    fn try_from(value: LiteralValue) -> Result<Self, Self::Error> {
        if let LiteralValue::String(string) = value {
            return Ok(string);
        }
        Err(error_reporter::Error {
            token: None,
            message: format!("{value} is not a string"),
        })
    }
}

impl TryFrom<LiteralValue> for bool {
    type Error = error_reporter::Error;

    fn try_from(value: LiteralValue) -> Result<Self, Self::Error> {
        match value {
            LiteralValue::Bool(value) => Ok(value),
            LiteralValue::String(_) | LiteralValue::Number(_) => Ok(true),
            LiteralValue::Nil => Ok(false),
        }
    }
}
