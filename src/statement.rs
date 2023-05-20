use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub enum Statement {
    Expression {
        expr: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Statement>,
    },
    Return {
        keyword: Token,
        expr: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expr,
        block: Box<Statement>,
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
