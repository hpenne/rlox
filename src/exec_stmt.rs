use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::result;

use crate::environment::Environment;
use crate::error_reporter;
use crate::evaluate_expr::EvaluateExpr;
use crate::literal_value::LiteralValue;
use crate::lox_callable::LoxCallable;
use crate::statement::Statement;

pub enum ErrorOrReturn {
    Error(error_reporter::Error),
    Return(LiteralValue),
}

pub type Result<T> = result::Result<T, ErrorOrReturn>;

pub trait ExecuteStatement<W>
where
    W: Write,
{
    fn execute(&self, environment: &Rc<RefCell<Environment>>, output: &mut W) -> Result<()>;
}

impl<W> ExecuteStatement<W> for Statement
where
    W: Write,
{
    fn execute(&self, environment: &Rc<RefCell<Environment>>, output: &mut W) -> Result<()> {
        match self {
            Statement::Expression { expr } => {
                expr.evaluate(environment, output)?;
            }
            Statement::Function { name, params, body } => (*environment).borrow_mut().define(
                name,
                LiteralValue::Function(LoxCallable::from_statement(
                    params.clone(),
                    (*body).clone(),
                )),
            )?,
            Statement::Return { expr, .. } => {
                return Err(ErrorOrReturn::Return(expr.evaluate(environment, output)?))
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(environment, output)? == LiteralValue::Bool(true) {
                    then_branch.execute(environment, output)?;
                } else if let Some(else_branch) = else_branch {
                    else_branch.execute(environment, output)?;
                }
            }
            Statement::While { condition, block } => {
                while condition.evaluate(environment, output)? == LiteralValue::Bool(true) {
                    block.execute(environment, output)?;
                }
            }
            Statement::Print { expr } => {
                let value = expr.evaluate(environment, output)?;
                writeln!(output, "{value}").expect("Write to output failed");
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
                    initializer.evaluate(environment, output)?
                } else {
                    LiteralValue::Nil
                };
                (**environment).borrow_mut().define(name, value)?;
            }
        }
        Ok(())
    }
}

impl From<error_reporter::Error> for ErrorOrReturn {
    fn from(error: error_reporter::Error) -> Self {
        ErrorOrReturn::Error(error)
    }
}
