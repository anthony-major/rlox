use std::{collections::HashMap, error::Error};

use crate::{
    interpreter::{LoxValue, RuntimeError},
    token::{Token, TokenKind},
};

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<&LoxValue, RuntimeError> {
        if let TokenKind::Identifier(id) = name.kind() {
            match self.values.get(id) {
                Some(value) => Ok(value),
                None => Err(RuntimeError::new(
                    name.clone(),
                    format!("Undefined variable '{}'", id),
                )),
            }
        } else {
            Err(RuntimeError::new(
                name.clone(),
                "Expected identifier to be passed to environment get".to_string(),
            ))
        }
    }
}
