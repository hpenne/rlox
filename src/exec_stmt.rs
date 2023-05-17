use crate::environment::Environment;
use crate::error_reporter;
use crate::evaluate_expr::EvaluateExpr;
use crate::expr::LiteralValue;
use crate::statement::Statement;

pub trait ExecuteStatement {
    fn execute(&self, environment: &mut Environment) -> error_reporter::Result<()>;
}

impl ExecuteStatement for Statement {
    fn execute(&self, environment: &mut Environment) -> error_reporter::Result<()> {
        use Statement::*;
        match self {
            Expression { expr } => {
                expr.evaluate(environment)?;
            }
            Print { expr } => {
                println!("{}", expr.evaluate(environment)?);
            }
            Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    initializer.evaluate(environment)?
                } else {
                    LiteralValue::Nil
                };
                environment.define(name.lexeme.clone(), value);
            }
        }
        Ok(())
    }
}
