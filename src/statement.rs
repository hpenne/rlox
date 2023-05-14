use crate::expr::Expr;

pub enum Statement {
    Expression { expr: Expr },
    Print { expr: Expr },
}
