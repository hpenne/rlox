use crate::error_reporter;
use crate::error_reporter::{Error, ErrorReporter};
use crate::expr::{Expr, LiteralValue};
use crate::token::Token;
use crate::token_type::TokenType;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser<I>
where
    I: Iterator<Item = Token> + Clone,
{
    tokens: I,
    error_reporter: Rc<RefCell<ErrorReporter>>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token> + Clone,
{
    pub fn new(tokens: I, error: Rc<RefCell<ErrorReporter>>) -> Self {
        Self {
            tokens,
            error_reporter: error,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn expression(&mut self) -> error_reporter::Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> error_reporter::Result<Expr> {
        let mut expr = self.comparison()?;
        while let Some(token_type) = self.peek_token_type() {
            match token_type {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: self.next_token().unwrap(),
                        right: Box::new(self.comparison()?),
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> error_reporter::Result<Expr> {
        let mut expr = self.term()?;
        while let Some(token_type) = self.peek_token_type() {
            use TokenType::*;
            match token_type {
                Greater | GreaterEqual | Less | LessEqual => {
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: self.next_token().unwrap(),
                        right: Box::new(self.term()?),
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> error_reporter::Result<Expr> {
        let mut expr = self.factor()?;
        while let Some(token_type) = self.peek_token_type() {
            match token_type {
                TokenType::Minus | TokenType::Plus => {
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: self.next_token().unwrap(),
                        right: Box::new(self.factor()?),
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> error_reporter::Result<Expr> {
        let mut expr = self.unary()?;
        while let Some(token_type) = self.peek_token_type() {
            match token_type {
                TokenType::Slash | TokenType::Star => {
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: self.next_token().unwrap(),
                        right: Box::new(self.unary()?),
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> error_reporter::Result<Expr> {
        if let Some(token_type) = self.peek_token_type() {
            match token_type {
                TokenType::Bang | TokenType::Minus => {
                    return Ok(Expr::Unary {
                        operator: self.next_token().unwrap(),
                        right: Box::new(self.unary()?),
                    })
                }
                _ => {}
            }
        }
        return self.primary();
    }

    fn primary(&mut self) -> error_reporter::Result<Expr> {
        let current = self.tokens.clone();
        match self.tokens.next() {
            Some(Token {
                token_type: TokenType::False,
                ..
            }) => Ok(Expr::Literal {
                value: LiteralValue::Bool(false),
            }),
            Some(Token {
                token_type: TokenType::True,
                ..
            }) => Ok(Expr::Literal {
                value: LiteralValue::Bool(true),
            }),
            Some(Token {
                token_type: TokenType::Nil,
                ..
            }) => Ok(Expr::Literal {
                value: LiteralValue::Nil,
            }),
            Some(Token {
                token_type: TokenType::Number,
                lexeme,
                ..
            }) => Ok(Expr::Literal {
                value: LiteralValue::Number(lexeme.parse().unwrap()),
            }),
            Some(Token {
                token_type: TokenType::String,
                lexeme,
                ..
            }) => Ok(Expr::Literal {
                value: LiteralValue::String(lexeme),
            }),
            Some(Token {
                token_type: TokenType::LeftParen,
                ..
            }) => {
                let expression = self.expression()?;
                let token = self.next_token();
                if token.as_ref().map(|token| token.token_type) == Some(TokenType::RightParen) {
                    Ok(Expr::Grouping {
                        expression: Box::new(expression),
                    })
                } else {
                    Err(self.error(token, "Expect ')' after expression."))
                }
            }
            None => {
                self.tokens = current;
                Err(self.error(None, "Unexpected end of file"))
            }
            _ => {
                self.tokens = current;
                Err(self.error(None, "Expected expression"))
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.tokens.clone().next()
    }

    fn peek_token_type(&mut self) -> Option<TokenType> {
        self.peek_token().map(|token| token.token_type)
    }

    fn error(&mut self, token: Option<Token>, message: &str) -> Error {
        self.error_reporter
            .borrow_mut()
            .error_with_token(token.clone(), &message);
        Error {
            token,
            message: message.into(),
        }
    }
}
