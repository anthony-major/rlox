use std::{error::Error, fmt::Display, io::Write};

use crate::{
    ast::{Accept, Visitor},
    scanner::{Scanner, ScannerError},
    token::TokenKind,
};

#[derive(Debug, PartialEq, Clone)]
pub enum LoxValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LoxValue::Nil => false,
            LoxValue::Boolean(b) => b.clone(),
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn run_file(path: &str) -> std::io::Result<()> {
        let interpreter = Self {};
        let code = std::fs::read_to_string(path)?;

        interpreter.run(&code);

        Ok(())
    }

    pub fn run_prompt() -> std::io::Result<()> {
        let interpreter = Self {};
        let mut input = String::new();

        loop {
            print!(">");
            std::io::stdout().flush()?;
            let bytes_read = std::io::stdin().read_line(&mut input)?;
            if bytes_read == 0 {
                break;
            }

            interpreter.run(&input);

            input.clear();
        }

        Ok(())
    }

    fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        let mut errors: Vec<ScannerError> = Vec::new();

        loop {
            match scanner.get_next_token() {
                Ok(token) => {
                    println!("{}", token);

                    if token.kind() == &TokenKind::Eof {
                        break;
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        for err in errors {
            println!("{}", err);
        }
    }
}

impl Visitor for Interpreter {
    type Result = Result<LoxValue, RuntimeError>;

    fn visit_literal(&self, literal: &crate::ast::Literal) -> Self::Result {
        match literal.value.kind() {
            TokenKind::Nil => Ok(LoxValue::Nil),
            TokenKind::True => Ok(LoxValue::Boolean(true)),
            TokenKind::False => Ok(LoxValue::Boolean(false)),
            TokenKind::Number(x) => Ok(LoxValue::Number(x.clone())),
            TokenKind::String(s) => Ok(LoxValue::String(s.clone())),
            _ => Err(RuntimeError {}),
        }
    }

    fn visit_grouping(&self, grouping: &crate::ast::Grouping) -> Self::Result {
        grouping.expression.accept(self)
    }

    fn visit_unary(&self, unary: &crate::ast::Unary) -> Self::Result {
        let right = unary.right.accept(self)?;

        match unary.operator.kind() {
            TokenKind::Minus => match right {
                LoxValue::Number(x) => Ok(LoxValue::Number(-x)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::Bang => Ok(LoxValue::Boolean(!right.is_truthy())),
            _ => Err(RuntimeError {}),
        }
    }

    fn visit_binary(&self, binary: &crate::ast::Binary) -> Self::Result {
        let left = binary.left.accept(self)?;
        let right = binary.right.accept(self)?;

        match binary.operator.kind() {
            TokenKind::Minus => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Number(x - y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::Slash => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Number(x / y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::Star => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Number(x * y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::Plus => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Number(x / y)),
                (LoxValue::String(x), LoxValue::String(y)) => Ok(LoxValue::String(x + &y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::Greater => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Boolean(x > y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::GreaterEqual => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Boolean(x >= y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::Less => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Boolean(x < y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::LessEqual => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Boolean(x <= y)),
                _ => Err(RuntimeError {}),
            },
            TokenKind::BangEqual => Ok(LoxValue::Boolean(left != right)),
            TokenKind::EqualEqual => Ok(LoxValue::Boolean(left == right)),
            _ => Err(RuntimeError {}),
        }
    }
}
