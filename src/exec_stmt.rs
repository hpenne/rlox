use crate::error_reporter;
use crate::evaluate_expr::EvaluateExpr;
use crate::statement::Statement;

pub trait ExecuteStatement {
    fn execute(&self) -> error_reporter::Result<()>;
}

impl ExecuteStatement for Statement {
    fn execute(&self) -> error_reporter::Result<()> {
        use Statement::*;
        match self {
            Expression { expr } => {
                expr.evaluate()?;
            }
            Print { expr } => {
                println!("{}", expr.evaluate()?);
            }
        }
        Ok(())
    }
}
