use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error_reporter;
use crate::error_reporter::Error;
use crate::literal_value::LiteralValue;
use crate::token::Token;

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn from_parent(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::default(),
            enclosing: Some(enclosing.clone()),
        }
    }

    pub fn define(&mut self, name: &Token, value: LiteralValue) -> error_reporter::Result<()> {
        match self.values.insert(name.lexeme.clone(), value) {
            None => Ok(()),
            Some(_) => Err(Error {
                token: Some(name.clone()),
                message: format!("Variable {name} already defined"),
            }),
        }
    }

    pub fn assign(&mut self, name: &Token, new_value: LiteralValue) -> error_reporter::Result<()> {
        if let Some(current_value) = self.values.get_mut(&name.lexeme) {
            *current_value = new_value;
            return Ok(());
        };

        if let Some(ref mut enclosing) = self.enclosing {
            return (**enclosing).borrow_mut().assign(name, new_value);
        }

        Err(Error {
            token: None,
            message: format!("Variable {name} not defined"),
        })
    }

    pub fn get(&self, name: &Token) -> error_reporter::Result<LiteralValue> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(Error {
            token: None,
            message: format!("Undefined variable {}", name.lexeme),
        })
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> error_reporter::Result<LiteralValue> {
        if distance == 0 {
            Ok(self.get(name)?)
        } else if let Some(enclosing) = &self.enclosing {
            (*enclosing).borrow().get_at(distance - 1, name)
        } else {
            panic!("Incorrect distance!")
        }
    }
}
