use std::fmt::{Display, Formatter};

use crate::literal_value::LiteralValue;
use crate::token::Token;

#[derive(Clone)]
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
    Call {
        callee: Box<Expr>,
        closing_paren: Token,
        arguments: Vec<Expr>,
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
            Expr::Call { .. } => write!(f, "call"),
            Expr::Grouping { expression } => write!(f, "(group {expression})"),
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator.lexeme, right),
            Expr::Variable { name } => write!(f, "{name}"),
        }
    }
}
