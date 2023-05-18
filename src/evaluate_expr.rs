use crate::environment::Environment;
use crate::error_reporter::{Error, Result};
use crate::expr::{Expr, LiteralValue};
use crate::token_type::TokenType;

pub trait EvaluateExpr {
    fn evaluate(&self, environment: &mut Environment) -> Result<LiteralValue>;
}

impl EvaluateExpr for Expr {
    fn evaluate(&self, environment: &mut Environment) -> Result<LiteralValue> {
        match self {
            Expr::Assign { name, expression } => {
                let value = expression.evaluate(environment)?;
                environment.assign(name, value.clone())?;
                Ok(value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment)?;
                let right = right.evaluate(environment)?;
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
                        if right != 0f64 {
                            Ok(LiteralValue::Number(f64::try_from(left)? / right))
                        } else {
                            Err(Error {
                                token: Some(operator.clone()),
                                message: "Division by 0".into(),
                            })
                        }
                    }
                    TokenType::Star => Ok(LiteralValue::Number(
                        f64::try_from(left)? * f64::try_from(right)?,
                    )),
                    TokenType::Greater => Ok(LiteralValue::Bool(
                        bool::try_from(left)? > bool::try_from(right)?,
                    )),
                    TokenType::GreaterEqual => Ok(LiteralValue::Bool(
                        bool::try_from(left)? >= bool::try_from(right)?,
                    )),
                    TokenType::Less => Ok(LiteralValue::Bool(
                        bool::try_from(left)? < bool::try_from(right)?,
                    )),
                    TokenType::LessEqual => Ok(LiteralValue::Bool(
                        bool::try_from(left)? <= bool::try_from(right)?,
                    )),
                    TokenType::EqualEqual => Ok(LiteralValue::Bool(is_equal(left, right))),
                    TokenType::BangEqual => Ok(LiteralValue::Bool(!is_equal(left, right))),
                    _ => {
                        panic!(
                            "Missing implementation for operator {}",
                            operator.token_type
                        );
                    }
                }
            }
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Variable { name } => environment.get(name),
            Expr::Unary { operator, right } => match operator.token_type {
                TokenType::Bang => {
                    let boolean_value: bool = right.evaluate(environment)?.try_into()?;
                    Ok(LiteralValue::Bool(!boolean_value))
                }
                TokenType::Minus => {
                    let number: f64 = right.evaluate(environment)?.try_into()?;
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

fn is_equal(left: LiteralValue, right: LiteralValue) -> bool {
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
