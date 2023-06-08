use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error_reporter::{Error, Result};
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::literal_value::LiteralValue;
use crate::token::Token;
use crate::token_type::TokenType;

pub trait EvaluateExpr {
    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        interpreter: &mut Interpreter,
    ) -> Result<LiteralValue>;
}

impl EvaluateExpr for Expr {
    #[allow(clippy::too_many_lines)]
    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        interpreter: &mut Interpreter,
    ) -> Result<LiteralValue> {
        match self {
            Expr::Assign { name, expression } => {
                let value = expression.evaluate(environment, interpreter)?;
                environment.borrow_mut().assign(name, value.clone())?;
                Ok(value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment, interpreter)?;
                let right = right.evaluate(environment, interpreter)?;
                match operator.token_type {
                    TokenType::Minus => Ok(LiteralValue::Number(
                        f64::try_from(left)? - f64::try_from(right)?,
                    )),
                    TokenType::Plus => match left {
                        LiteralValue::Number(left) => {
                            Ok(LiteralValue::Number(left + f64::try_from(right)?))
                        }
                        LiteralValue::String(mut left) => {
                            left.push_str(try_into_str(&right)?);
                            Ok(LiteralValue::String(left))
                        }
                        _ => Err(Error {
                            token: None,
                            message: "Operands must be two numbers or two strings".into(),
                        }),
                    },
                    TokenType::Slash => {
                        let right = f64::try_from(right)?;
                        if right == 0f64 {
                            Err(Error {
                                token: Some(operator.clone()),
                                message: "Division by 0".into(),
                            })
                        } else {
                            Ok(LiteralValue::Number(f64::try_from(left)? / right))
                        }
                    }
                    TokenType::Star => Ok(LiteralValue::Number(
                        f64::try_from(left)? * f64::try_from(right)?,
                    )),
                    TokenType::Greater => Ok(LiteralValue::Bool(
                        f64::try_from(left)? > f64::try_from(right)?,
                    )),
                    TokenType::GreaterEqual => Ok(LiteralValue::Bool(
                        f64::try_from(left)? >= f64::try_from(right)?,
                    )),
                    TokenType::Less => Ok(LiteralValue::Bool(
                        f64::try_from(left)? < f64::try_from(right)?,
                    )),
                    TokenType::LessEqual => Ok(LiteralValue::Bool(
                        f64::try_from(left)? <= f64::try_from(right)?,
                    )),
                    TokenType::EqualEqual => Ok(LiteralValue::Bool(is_equal(&left, &right))),
                    TokenType::BangEqual => Ok(LiteralValue::Bool(!is_equal(&left, &right))),
                    _ => panic!(
                        "Missing implementation for operator {}",
                        operator.token_type
                    ),
                }
            }
            Expr::Call {
                callee,
                closing_paren,
                arguments,
            } => {
                let callee_value = callee.evaluate(environment, interpreter)?;
                let argument_values = arguments
                    .iter()
                    .map(|arg| arg.evaluate(environment, interpreter))
                    .collect::<Result<Vec<_>>>()?;
                if let LiteralValue::Function(func) = callee_value {
                    if func.arity() == argument_values.len() {
                        Ok(func.call(argument_values, environment, interpreter)?)
                    } else {
                        Err(Error {
                            token: Some(closing_paren.clone()),
                            message: format!(
                                "Wrong number of arguments to function. Got {} but function requires {}",
                                argument_values.len(),
                                func.arity()
                            ),
                        })
                    }
                } else {
                    Err(Error {
                        token: Some(closing_paren.clone()),
                        message: "Can only call functions".into(),
                    })
                }
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left: bool = left.evaluate(environment, interpreter)?.try_into()?;
                match operator.token_type {
                    TokenType::Or => {
                        if left {
                            Ok(LiteralValue::Bool(true))
                        } else {
                            Ok(LiteralValue::Bool(
                                right.evaluate(environment, interpreter)?.try_into()?,
                            ))
                        }
                    }
                    TokenType::And => {
                        if left {
                            Ok(LiteralValue::Bool(
                                right.evaluate(environment, interpreter)?.try_into()?,
                            ))
                        } else {
                            Ok(LiteralValue::Bool(false))
                        }
                    }
                    _ => panic!("Unsupported binary operator: {}", operator.token_type),
                }
            }
            Expr::Grouping { expression } => expression.evaluate(environment, interpreter),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Variable { name } => lookup_variable(environment, interpreter, name),
            Expr::Unary { operator, right } => match operator.token_type {
                TokenType::Bang => {
                    let boolean_value: bool =
                        right.evaluate(environment, interpreter)?.try_into()?;
                    Ok(LiteralValue::Bool(!boolean_value))
                }
                TokenType::Minus => {
                    let number: f64 = right.evaluate(environment, interpreter)?.try_into()?;
                    Ok(LiteralValue::Number(-number))
                }
                _ => {
                    panic!(
                        "Missing implementation for operator {}",
                        operator.token_type
                    );
                }
            },
        }
    }
}

fn lookup_variable(
    environment: &Rc<RefCell<Environment>>,
    interpreter: &Interpreter,
    name: &Token,
) -> Result<LiteralValue> {
    if let Some(distance) = interpreter.resolver.get(name) {
        (**environment).borrow().get_at(*distance, name)
    } else {
        (*interpreter.globals).borrow().get(name)
    }
}

fn is_equal(left: &LiteralValue, right: &LiteralValue) -> bool {
    if matches!(left, LiteralValue::Nil) {
        return matches!(right, LiteralValue::Nil);
    }
    left == right
}

fn try_into_str(value: &LiteralValue) -> Result<&str> {
    if let LiteralValue::String(string) = value {
        return Ok(string.as_ref());
    }
    Err(Error {
        token: None,
        message: format!("{value} is not a string"),
    })
}
