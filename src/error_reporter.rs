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

    pub fn report(&mut self, line: usize, loc: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, loc, message);
        self.has_error = true;
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }
}
