use crate::token_type::TokenType;
use crate::{ErrorReporter, Token};
use std::cell::RefCell;
use std::rc::Rc;

pub trait TokenScanner<I>
where
    I: Iterator<Item = char> + Clone,
{
    fn tokens(self, error: Rc<RefCell<ErrorReporter>>) -> Scanner<I>;
}

impl<I> TokenScanner<I> for I
where
    I: Iterator<Item = char> + Clone,
{
    fn tokens(self, error: Rc<RefCell<ErrorReporter>>) -> Scanner<I> {
        Scanner::new(self, error)
    }
}

#[derive(Clone)]
pub struct Scanner<I>
where
    I: Iterator<Item = char> + Clone,
{
    source: I,
    error_reporter: Rc<RefCell<ErrorReporter>>,
    line: usize,
    count: usize,
}

impl<I> Scanner<I>
where
    I: Iterator<Item = char> + Clone,
{
    pub fn new(source: I, error: Rc<RefCell<ErrorReporter>>) -> Self {
        Self {
            source,
            error_reporter: error,
            line: 1,
            count: 0,
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

    fn peek(&mut self) -> char {
        let current = self.source.clone();
        if let Some(next) = self.source.next() {
            self.source = current;
            return next;
        }
        '\0'
    }

    fn token(&mut self, token_type: TokenType) -> Token {
        self.count += 1;
        Token::new(token_type, String::new(), self.line, self.count)
    }

    fn token_with_lexeme(&mut self, token_type: TokenType, lexeme: String) -> Token {
        self.count += 1;
        Token::new(token_type, lexeme, self.line, self.count)
    }

    fn consume_line(&mut self) {
        for c in self.source.by_ref() {
            if c == '\n' {
                self.line += 1;
                break;
            }
        }
    }

    fn string_literal(&mut self) -> Option<String> {
        let mut literal = String::default();
        loop {
            if let Some(c) = self.source.next() {
                match c {
                    '\n' => {
                        self.line += 1;
                    }
                    '"' => return Some(literal),
                    _ => {
                        literal.push(c);
                    }
                }
            } else {
                self.error_reporter
                    .borrow_mut()
                    .error(self.line, "Unterminated string");
                return None;
            }
        }
    }

    fn number(&mut self, c: char) -> Option<String> {
        let mut number: String = c.into();
        loop {
            let previous = self.source.clone();
            match self.source.next() {
                Some(c) if c.is_numeric() => number.push(c),
                Some('.') if self.peek().is_numeric() => number.push(c),
                Some(_) => {
                    self.source = previous;
                    return Some(number);
                }
                None => {
                    return Some(number);
                }
            }
        }
    }

    fn identifier(&mut self, c: char) -> Option<String> {
        let mut identifier: String = c.into();
        loop {
            let previous = self.source.clone();
            match self.source.next() {
                Some(c) if is_identifier_char(c) => identifier.push(c),
                Some(_) => {
                    self.source = previous;
                    return Some(identifier);
                }
                None => {
                    return Some(identifier);
                }
            }
        }
    }

    fn reserved_word_token(&mut self, identifier: &str) -> Option<Token> {
        if let Some(token_type) = reserved_word_token_type(identifier) {
            return Some(self.token(token_type));
        }
        None
    }
}

impl<I> Iterator for Scanner<I>
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

                // Slash and comments
                Some('/') => {
                    if self.match_next('/') {
                        self.consume_line();
                    } else {
                        return Some(self.token(TokenType::Slash));
                    }
                }

                // Whitespace etc:
                Some(' ' | '\r' | '\t') => {}
                Some('\n') => {
                    self.line += 1;
                }

                // Strings
                Some('"') => {
                    if let Some(s) = self.string_literal() {
                        return Some(self.token_with_lexeme(TokenType::String, s));
                    }
                }

                // Numbers
                Some(c) if c.is_numeric() => {
                    if let Some(number) = self.number(c) {
                        return Some(self.token_with_lexeme(TokenType::Number, number));
                    }
                }

                Some(c) if is_identifier_char(c) => {
                    if let Some(identifier) = self.identifier(c) {
                        if let Some(token) = self.reserved_word_token(&identifier) {
                            return Some(token);
                        }
                        return Some(self.token_with_lexeme(TokenType::Identifier, identifier));
                    }
                }

                // Identifiers and reserved words
                None => return None,
                _ => self
                    .error_reporter
                    .borrow_mut()
                    .error(self.line, "Unexpected character"),
            }
        }
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_numeric() || c.is_ascii_alphabetic()
}

fn reserved_word_token_type(identifier: &str) -> Option<TokenType> {
    match identifier {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "while" => Some(TokenType::While),
        "var" => Some(TokenType::Var),
        _ => None,
    }
}
