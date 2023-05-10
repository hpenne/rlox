use crate::error_reporter::ErrorReporter;
use crate::expr::{Expr, LiteralValue};
use crate::token::Token;
use crate::token_type::TokenType;
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    token: Option<Token>,
    message: String,
}

pub struct Parser<'a, I>
where
    I: Iterator<Item = Token> + Clone,
{
    tokens: I,
    error_reporter: &'a mut ErrorReporter,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token> + Clone,
{
    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
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

    fn comparison(&mut self) -> Result<Expr> {
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

    fn term(&mut self) -> Result<Expr> {
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

    fn factor(&mut self) -> Result<Expr> {
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

    fn unary(&mut self) -> Result<Expr> {
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

    fn primary(&mut self) -> Result<Expr> {
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
                let expr = self.expression()?;
                if self.peek_token_type() == Some(TokenType::RightParen) {
                    Ok(expr)
                } else {
                    Err(Error {
                        token: self.next_token(),
                        message: "Expect ')' after expression.".to_string(),
                    })
                }
            }
            _ => {
                self.tokens = current;
                Err(Error {
                    token: None,
                    message: "Unexpected end of file".to_string(),
                })
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.clone().next()
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.tokens.clone().next()
    }

    fn peek_token_type(&mut self) -> Option<TokenType> {
        self.peek_token().map(|token| token.token_type)
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_type() == Some(token_type) {
            self.next_token();
            return true;
        }
        false
    }
}
