// use std::collections::HashMap;
use std::ptr::null_mut;

use crate::error_reporter;
use crate::error_reporter::Error;
use crate::literal_value::LiteralValue;
use crate::token::Token;
use rustc_hash::FxHashMap as HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: *mut Environment,
    globals: *mut Environment,
}

impl Environment {
    /// Creates a new Environment enveloping a parent.
    ///
    /// Safety: The caller must ensure that the parent (`enclosing`) lives longer than the
    /// environment that is created.
    ///
    /// # Arguments
    ///
    /// * `enclosing`: The parent to enclose
    ///
    /// returns: Environment
    ///
    pub unsafe fn from_parent(enclosing: &mut Environment) -> Self {
        Self {
            values: HashMap::default(),
            enclosing: &mut (*enclosing),
            globals: enclosing.globals,
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

        unsafe {
            if !self.enclosing.is_null() {
                return (*self.enclosing).assign(name, new_value);
            }
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

        unsafe {
            if !self.enclosing.is_null() {
                return (*self.enclosing).get(name);
            }
        }

        Err(Error {
            token: None,
            message: format!("Undefined variable {}", name.lexeme),
        })
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> error_reporter::Result<LiteralValue> {
        if distance == 0 {
            Ok(self.get(name)?)
        } else {
            unsafe {
                if self.enclosing.is_null() {
                    panic!("Incorrect distance!")
                } else {
                    (*self.enclosing).get_at(distance - 1, name)
                }
            }
        }
    }

    pub fn get_global(&self, name: &Token) -> error_reporter::Result<LiteralValue> {
        unsafe {
            if self.globals.is_null() {
                self.get(name)
            } else {
                (*self.globals).get(name)
            }
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            values: Default::default(),
            enclosing: null_mut(),
            globals: null_mut(),
        }
    }
}
