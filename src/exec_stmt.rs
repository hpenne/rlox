use crate::environment::Environment;
use crate::error_reporter;
use crate::evaluate_expr::EvaluateExpr;
use crate::expr::LiteralValue;
use crate::statement::Statement;
use std::io::Write;

pub trait ExecuteStatement<W>
where
    W: Write,
{
    fn execute(&self, environment: &mut Environment, output: &mut W) -> error_reporter::Result<()>;
}

impl<W> ExecuteStatement<W> for Statement
where
    W: Write,
{
    fn execute(&self, environment: &mut Environment, output: &mut W) -> error_reporter::Result<()> {
        match self {
            Statement::Expression { expr } => {
                expr.evaluate(environment)?;
            }
            Statement::Print { expr } => {
                writeln!(output, "{}", expr.evaluate(environment)?)
                    .expect("Write to output failed");
                output.flush().unwrap();
            }
            Statement::Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    initializer.evaluate(environment)?
                } else {
                    LiteralValue::Nil
                };
                environment.define(&name.lexeme, value)?;
            }
        }
        Ok(())
    }
}
