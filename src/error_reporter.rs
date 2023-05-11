use crate::token::Token;

#[derive(Clone)]
pub struct ErrorReporter {
    has_error: bool,
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self { has_error: false }
    }
}

impl ErrorReporter {
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn error_with_token(&mut self, token: Option<Token>, message: &str) {
        if let Some(token) = token {
            self.report(
                token.line,
                format!(" at '{}' ", &token.lexeme).as_ref(),
                message,
            );
        } else {
            println!("Error: {}", message);
        }
    }

    pub fn report(&mut self, line: usize, loc: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, loc, message);
        self.has_error = true;
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }
}
