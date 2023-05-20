use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error_reporter::Result;
use crate::exec_stmt::ExecuteStatement;
use crate::literal_value::LiteralValue;
use crate::statement::Statement;
use crate::token::Token;

pub type LoxCallableFn =
    dyn Fn(Vec<LiteralValue>, &Rc<RefCell<Environment>>, &mut dyn Write) -> Result<LiteralValue>;

#[derive(Clone)]
pub struct LoxCallable {
    func: Rc<LoxCallableFn>,
    num_arguments: usize,
}

impl LoxCallable {
    pub fn from_fn(func: Rc<LoxCallableFn>, num_arguments: usize) -> Self {
        Self {
            func,
            num_arguments,
        }
    }

    pub fn from_statement(params: Vec<Token>, body: Vec<Statement>) -> Self {
        let num_arguments = params.len();
        Self {
            func: Rc::new(move |args, env, mut output| {
                let environment = Rc::new(RefCell::new(Environment::from_parent(env)));
                for (param, arg) in params.iter().zip(args.into_iter()) {
                    (*environment).borrow_mut().define(param, arg)?;
                }
                for statement in &body {
                    statement.execute(&environment, &mut output)?;
                }
                Ok(LiteralValue::Nil)
            }),
            num_arguments,
        }
    }

    pub fn call(
        &self,
        arguments: Vec<LiteralValue>,
        environment: &Rc<RefCell<Environment>>,
        output: &mut dyn Write,
    ) -> Result<LiteralValue> {
        (self.func)(arguments, environment, output)
    }

    pub fn arity(&self) -> usize {
        self.num_arguments
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Display for LoxCallable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<some function>")
    }
}

impl Debug for LoxCallable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
