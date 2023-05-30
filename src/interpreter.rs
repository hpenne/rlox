use std::cell::RefCell;
use crate::resolver::ResolveLookup;
use std::io::Write;
use std::rc::Rc;
use crate::environment::Environment;

pub struct Interpreter<'a> {
    pub globals: Rc<RefCell<Environment>>,
    pub resolver: ResolveLookup,
    pub output: &'a mut dyn Write,
}
