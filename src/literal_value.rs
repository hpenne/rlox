use std::fmt::{Display, Formatter};

use crate::error_reporter;
use crate::lox_callable::LoxCallable;

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Bool(bool),
    String(String),
    Number(f64),
    Function(LoxCallable),
    Nil,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Bool(value) => write!(f, "{value}"),
            LiteralValue::String(value) => write!(f, "{value}"),
            LiteralValue::Number(value) => write!(f, "{value}"),
            LiteralValue::Function(func) => write!(f, "{func}"),
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
            LiteralValue::String(_) | LiteralValue::Number(_) | LiteralValue::Function(_) => {
                Ok(true)
            }
            LiteralValue::Nil => Ok(false),
        }
    }
}
