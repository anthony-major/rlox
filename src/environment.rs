use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use crate::{
    interpreter::{LoxValue, RuntimeError},
    token::{Token, TokenKind},
};

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            ..Default::default()
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxValue, Box<dyn Error>> {
        if let TokenKind::Identifier(id) = name.kind() {
            match self.values.get(id) {
                Some(value) => Ok(value.clone()),
                None => match &self.enclosing {
                    Some(enclosing) => enclosing.borrow_mut().get(name),
                    None => Err(Box::new(RuntimeError::new(
                        name.clone(),
                        format!("Undefined variable '{}'", id),
                    ))),
                },
            }
        } else {
            Err(Box::new(RuntimeError::new(
                name.clone(),
                "Expected identifier to be passed to environment get".to_string(),
            )))
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<LoxValue, Box<dyn Error>> {
        if let TokenKind::Identifier(id) = name.kind() {
            if self.values.contains_key(id) {
                *self.values.get_mut(id).unwrap() = value.clone();
                Ok(value)
            } else {
                match &self.enclosing {
                    Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                    None => Err(Box::new(RuntimeError::new(
                        name.clone(),
                        format!("Undefined variable '{}'", id),
                    ))),
                }
            }
        } else {
            Err(Box::new(RuntimeError::new(
                name.clone(),
                "Expected identifier to be passed to environment assign".to_string(),
            )))
        }
    }
}
