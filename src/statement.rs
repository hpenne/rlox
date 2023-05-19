use crate::expr::Expr;
use crate::token::Token;

pub enum Statement {
    Expression {
        expr: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print {
        expr: Expr,
    },
    Block {
        statements: Vec<Statement>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}
