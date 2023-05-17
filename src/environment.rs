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
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&LiteralValue> {
        self.values.get(name)
    }
}
