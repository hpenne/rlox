use crate::error_reporter::{Error, Result};
use crate::expr::{Expr, LiteralValue};
use crate::token_type::TokenType;

pub trait EvaluateExpr {
    fn evaluate(&self) -> Result<LiteralValue>;
}

impl EvaluateExpr for Expr {
    fn evaluate(&self) -> Result<LiteralValue> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => match operator.token_type {
                TokenType::Minus => {
                    let left: f64 = left.evaluate()?.try_into()?;
                    let right: f64 = right.evaluate()?.try_into()?;
                    Ok(LiteralValue::Number(left - right))
                }
                TokenType::Plus => {
                    let left: f64 = left.evaluate()?.try_into()?;
                    let right: f64 = right.evaluate()?.try_into()?;
                    Ok(LiteralValue::Number(left + right))
                }
                TokenType::Slash => {
                    let left: f64 = left.evaluate()?.try_into()?;
                    let right: f64 = right.evaluate()?.try_into()?;
                    if right != 0f64 {
                        Ok(LiteralValue::Number(left / right))
                    } else {
                        Err(Error {
                            token: None,
                            message: "Division by 0".into(),
                        })
                    }
                }
                TokenType::Star => {
                    let left: f64 = left.evaluate()?.try_into()?;
                    let right: f64 = right.evaluate()?.try_into()?;
                    Ok(LiteralValue::Number(left * right))
                }
                _ => {
                    panic!(
                        "Missing implementatoin for operator {}",
                        operator.token_type
                    );
                }
            },
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Unary { operator, right } => match operator.token_type {
                TokenType::Bang => {
                    let boolean_value: bool = right.evaluate()?.try_into()?;
                    Ok(LiteralValue::Bool(!boolean_value))
                }
                TokenType::Minus => {
                    let number: f64 = right.evaluate()?.try_into()?;
                    Ok(LiteralValue::Number(-number))
                }
                _ => {
                    panic!(
                        "Missing implementatoin for operator {}",
                        operator.token_type
                    );
                }
            },
        }
    }
}
