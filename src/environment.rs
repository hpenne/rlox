use crate::error_reporter;
use crate::error_reporter::Error;
use crate::expr::LiteralValue;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            values: HashMap::default(),
        }
    }
}

impl Environment {
    pub fn define(&mut self, name: &str, value: LiteralValue) -> error_reporter::Result<()> {
        match self.values.insert(name.to_string(), value) {
            None => Ok(()),
            Some(_) => Err(Error {
                token: None,
                message: format!("Variable {name} already defined").to_string(),
            }),
        }
    }

    pub fn assign(&mut self, name: &str, new_value: LiteralValue) -> error_reporter::Result<()> {
        if let Some(current_value) = self.values.get_mut(name) {
            *current_value = new_value;
            Ok(())
        } else {
            Err(Error {
                token: None,
                message: format!("Variable {name} not defined").to_string(),
            })
        }
    }

    pub fn get(&self, name: &str) -> error_reporter::Result<LiteralValue> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else {
            Err(Error {
                token: None,
                message: format!("Undefined variable {name}"),
            })
        }
    }
}
