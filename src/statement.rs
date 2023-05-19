use crate::expr::Expr;
use crate::token::Token;

pub enum Statement {
    Print {
        expr: Expr,
    },
    Block {
        statements: Vec<Statement>,  
    },
    Expression {
        expr: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}
