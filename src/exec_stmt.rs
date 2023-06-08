use std::cell::RefCell;
use std::rc::Rc;
use std::result;

use crate::environment::Environment;
use crate::error_reporter;
use crate::evaluate_expr::EvaluateExpr;
use crate::interpreter::Interpreter;
use crate::literal_value::LiteralValue;
use crate::lox_callable::LoxCallable;
use crate::statement::Statement;

pub enum ErrorOrReturn {
    Error(error_reporter::Error),
    Return(LiteralValue),
}

pub type Result<T> = result::Result<T, ErrorOrReturn>;

pub trait ExecuteStatement {
    fn execute(
        &self,
        environment: &Rc<RefCell<Environment>>,
        interpreter: &mut Interpreter,
    ) -> Result<()>;
}

impl ExecuteStatement for Statement {
    fn execute(
        &self,
        environment: &Rc<RefCell<Environment>>,
        interpreter: &mut Interpreter,
    ) -> Result<()> {
        match self {
            Statement::Expression { expr } => {
                expr.evaluate(environment, interpreter)?;
            }
            Statement::Function { name, params, body } => (*environment).borrow_mut().define(
                name,
                LiteralValue::Function(LoxCallable::from_statement(
                    params.clone(),
                    (*body).clone(),
                )),
            )?,
            Statement::Return { expr, .. } => {
                return Err(ErrorOrReturn::Return(if let Some(expr) = expr {
                    expr.evaluate(environment, interpreter)?
                } else {
                    LiteralValue::Nil
                }))
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(environment, interpreter)? == LiteralValue::Bool(true) {
                    then_branch.execute(environment, interpreter)?;
                } else if let Some(else_branch) = else_branch {
                    else_branch.execute(environment, interpreter)?;
                }
            }
            Statement::While { condition, block } => {
                while condition.evaluate(environment, interpreter)? == LiteralValue::Bool(true) {
                    block.execute(environment, interpreter)?;
                }
            }
            Statement::Print { expr } => {
                let value = expr.evaluate(environment, interpreter)?;
                writeln!(interpreter.output, "{value}").expect("Write to output failed");
                interpreter.output.flush().unwrap();
            }
            Statement::Block { statements } => {
                let block_env = Rc::new(RefCell::new(Environment::from_parent(environment)));
                for statement in statements {
                    statement.execute(&block_env, interpreter)?;
                }
            }
            Statement::Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    initializer.evaluate(environment, interpreter)?
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
