use crate::token_type::TokenType;
use crate::{Error, Token};

pub trait TokenScanner<'a, I>
where
    I: Iterator<Item = char> + Clone,
{
    fn tokens(self, error: &'a mut Error) -> Scanner<'a, I>;
}

impl<'a, I> TokenScanner<'a, I> for I
where
    I: Iterator<Item = char> + Clone,
{
    fn tokens(self, error: &'a mut Error) -> Scanner<'a, I> {
        Scanner::new(self, error)
    }
}

pub struct Scanner<'a, I>
where
    I: Iterator<Item = char> + Clone,
{
    source: I,
    error: &'a mut Error,
    line: usize,
}

impl<'a, I> Scanner<'a, I>
where
    I: Iterator<Item = char> + Clone,
{
    pub fn new(source: I, error: &'a mut Error) -> Self {
        Self {
            source,
            error,
            line: 1,
        }
    }

    fn match_next(&mut self, c: char) -> bool {
        let current = self.source.clone();
        if let Some(next) = self.source.next() {
            if next == c {
                return true;
            }
        }
        self.source = current;
        false
    }

    fn token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, "".to_string(), self.line)
    }
}

impl<I> Iterator for Scanner<'_, I>
where
    I: Iterator<Item = char> + Clone,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.source.next() {
                // Single character tokens:
                Some('(') => return Some(self.token(TokenType::LeftParen)),
                Some(')') => return Some(self.token(TokenType::RightParen)),
                Some('{') => return Some(self.token(TokenType::LeftBrace)),
                Some('}') => return Some(self.token(TokenType::RightBrace)),
                Some(',') => return Some(self.token(TokenType::Comma)),
                Some('.') => return Some(self.token(TokenType::Dot)),
                Some('-') => return Some(self.token(TokenType::Minus)),
                Some('+') => return Some(self.token(TokenType::Plus)),
                Some(';') => return Some(self.token(TokenType::Semicolon)),
                Some('*') => return Some(self.token(TokenType::Star)),

                // Two character tokens:
                Some('!') => {
                    return if self.match_next('=') {
                        Some(self.token(TokenType::BangEqual))
                    } else {
                        Some(self.token(TokenType::Bang))
                    }
                }
                Some('=') => {
                    return if self.match_next('=') {
                        Some(self.token(TokenType::EqualEqual))
                    } else {
                        Some(self.token(TokenType::Equal))
                    }
                }
                Some('<') => {
                    return if self.match_next('=') {
                        Some(self.token(TokenType::LessEqual))
                    } else {
                        Some(self.token(TokenType::Less))
                    }
                }
                Some('>') => {
                    return if self.match_next('=') {
                        Some(self.token(TokenType::GreaterEqual))
                    } else {
                        Some(self.token(TokenType::Greater))
                    }
                }

                None => return None,
                _ => self.error.error(self.line, "Unexpected character"),
            }
        }
    }
}
