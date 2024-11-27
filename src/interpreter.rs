use std::{error::Error, fmt::Display};

use crate::{
    ast::{Expr, ExprAccept, ExprVisitor},
    token::{Token, TokenKind},
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

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{}", s),
            Self::Number(x) => write!(f, "{}", x.to_string().trim_end_matches(".0")),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {} at '{}': {}",
            self.token.line(),
            self.token.kind(),
            self.message
        )
    }
}

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expression: Expr) -> Result<LoxValue, Box<dyn Error>> {
        expression
            .accept(self)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

fn evaluate_number_operands<F: Fn(f64, f64) -> LoxValue>(
    operator: Token,
    left: LoxValue,
    right: LoxValue,
    operation: F,
) -> Result<LoxValue, RuntimeError> {
    match (left, right) {
        (LoxValue::Number(x), LoxValue::Number(y)) => Ok(operation(x, y)),
        _ => Err(RuntimeError::new(
            operator,
            "Expected two numbers for binary operator".to_string(),
        )),
    }
}

impl ExprVisitor for Interpreter {
    type Result = Result<LoxValue, RuntimeError>;

    fn visit_literal(&self, literal: &crate::ast::Literal) -> Self::Result {
        match literal.value.kind() {
            TokenKind::Nil => Ok(LoxValue::Nil),
            TokenKind::True => Ok(LoxValue::Boolean(true)),
            TokenKind::False => Ok(LoxValue::Boolean(false)),
            TokenKind::Number(x) => Ok(LoxValue::Number(x.clone())),
            TokenKind::String(s) => Ok(LoxValue::String(s.clone())),
            _ => Err(RuntimeError::new(
                literal.value.clone(),
                "Expected literal".to_string(),
            )),
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
                _ => Err(RuntimeError::new(
                    unary.operator.clone(),
                    "Expected number after unary operator".to_string(),
                )),
            },
            TokenKind::Bang => Ok(LoxValue::Boolean(!right.is_truthy())),
            _ => Err(RuntimeError::new(
                unary.operator.clone(),
                "Expected unary operator".to_string(),
            )),
        }
    }

    fn visit_binary(&self, binary: &crate::ast::Binary) -> Self::Result {
        let left = binary.left.accept(self)?;
        let right = binary.right.accept(self)?;

        match binary.operator.kind() {
            TokenKind::Minus => {
                evaluate_number_operands(binary.operator.clone(), left, right, |x, y| {
                    LoxValue::Number(x - y)
                })
            }
            TokenKind::Slash => {
                evaluate_number_operands(binary.operator.clone(), left, right, |x, y| {
                    LoxValue::Number(x / y)
                })
            }
            TokenKind::Star => {
                evaluate_number_operands(binary.operator.clone(), left, right, |x, y| {
                    LoxValue::Number(x * y)
                })
            }
            TokenKind::Plus => match (left, right) {
                (LoxValue::Number(x), LoxValue::Number(y)) => Ok(LoxValue::Number(x + y)),
                (LoxValue::String(x), LoxValue::String(y)) => Ok(LoxValue::String(x + &y)),
                _ => Err(RuntimeError::new(
                    binary.operator.clone(),
                    "Expected two numbers or two strings".to_string(),
                )),
            },
            TokenKind::Greater => {
                evaluate_number_operands(binary.operator.clone(), left, right, |x, y| {
                    LoxValue::Boolean(x > y)
                })
            }
            TokenKind::GreaterEqual => {
                evaluate_number_operands(binary.operator.clone(), left, right, |x, y| {
                    LoxValue::Boolean(x >= y)
                })
            }
            TokenKind::Less => {
                evaluate_number_operands(binary.operator.clone(), left, right, |x, y| {
                    LoxValue::Boolean(x < y)
                })
            }
            TokenKind::LessEqual => {
                evaluate_number_operands(binary.operator.clone(), left, right, |x, y| {
                    LoxValue::Boolean(x <= y)
                })
            }
            TokenKind::BangEqual => Ok(LoxValue::Boolean(left != right)),
            TokenKind::EqualEqual => Ok(LoxValue::Boolean(left == right)),
            _ => Err(RuntimeError::new(
                binary.operator.clone(),
                "Expected binary operator".to_string(),
            )),
        }
    }
}
