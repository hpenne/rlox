use crate::environment::Environment;
use crate::error_reporter;
use crate::evaluate_expr::EvaluateExpr;
use crate::expr::LiteralValue;
use crate::statement::Statement;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

pub trait ExecuteStatement<W>
where
    W: Write,
{
    fn execute(
        &self,
        environment: &Rc<RefCell<Environment>>,
        output: &mut W,
    ) -> error_reporter::Result<()>;
}

impl<W> ExecuteStatement<W> for Statement
where
    W: Write,
{
    fn execute(
        &self,
        environment: &Rc<RefCell<Environment>>,
        output: &mut W,
    ) -> error_reporter::Result<()> {
        match self {
            Statement::Expression { expr } => {
                expr.evaluate(environment)?;
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(environment)? == LiteralValue::Bool(true) {
                    then_branch.execute(environment, output)?;
                } else if let Some(else_branch) = else_branch {
                    else_branch.execute(environment, output)?;
                }
            }
            Statement::While { condition, block } => {
                while condition.evaluate(environment)? == LiteralValue::Bool(true) {
                    block.execute(environment, output)?;
                }
            }
            Statement::Print { expr } => {
                writeln!(output, "{}", expr.evaluate(environment)?)
                    .expect("Write to output failed");
                output.flush().unwrap();
            }
            Statement::Block { statements } => {
                let block_env = Rc::new(RefCell::new(Environment::from_parent(environment)));
                for statement in statements {
                    statement.execute(&block_env, output)?;
                }
            }
            Statement::Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    initializer.evaluate(environment)?
                } else {
                    LiteralValue::Nil
                };
                (**environment).borrow_mut().define(name, value)?;
            }
        }
        Ok(())
    }
}
