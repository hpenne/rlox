use std::rc::Rc;
use std::time::SystemTime;

use crate::environment::Environment;
use crate::literal_value::LiteralValue;
use crate::lox_callable::LoxCallable;
use crate::token::Token;
use crate::token_type::TokenType;

#[allow(clippy::cast_precision_loss)]
pub fn add_builtin_functions(environment: &mut Environment) {
    environment
        .define(
            &Token {
                token_type: TokenType::Fun,
                lexeme: "clock".to_string(),
                line: 0,
                count: 0,
            },
            LiteralValue::Function(LoxCallable::from_fn(
                Rc::new(|_args, _env, _out| {
                    Ok(LiteralValue::Number(
                        (SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_micros() as f64)
                            / 1_000_000.0,
                    ))
                }),
                0,
            )),
        )
        .unwrap();
}
