use crate::resolver::ResolveLookup;
use std::io::Write;

pub struct Interpreter<'a> {
    pub resolver: ResolveLookup,
    pub output: &'a mut dyn Write,
}
