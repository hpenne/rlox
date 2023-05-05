pub struct Error {
    has_error: bool,
}

impl Default for Error {
    fn default() -> Self {
        Self { has_error: false }
    }
}

impl Error {
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

    pub fn reset_error(&mut self) {
        self.has_error = false;
    }
}
