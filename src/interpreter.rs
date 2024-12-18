use std::{error::Error, fmt::Display};

use crate::{
    ast::{Expr, ExprAccept, ExprVisitor, Stmt, StmtAccept, StmtVisitor},
    environment::Environment,
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
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), Box<dyn Error>> {
        for statement in statements {
            statement.accept(self)?;
        }

        Ok(())
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

    fn visit_literal(&mut self, literal: &crate::ast::Literal) -> Self::Result {
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

    fn visit_grouping(&mut self, grouping: &crate::ast::Grouping) -> Self::Result {
        grouping.expression.accept(self)
    }

    fn visit_unary(&mut self, unary: &crate::ast::Unary) -> Self::Result {
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

    fn visit_binary(&mut self, binary: &crate::ast::Binary) -> Self::Result {
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

    fn visit_variable(&mut self, variable: &crate::ast::Variable) -> Self::Result {
        match self.environment.get(&variable.name) {
            Ok(value) => Ok(value.clone()),
            Err(err) => Err(err),
        }
    }

    fn visit_assign(&mut self, assign: &crate::ast::Assign) -> Self::Result {
        todo!()
    }
}

impl StmtVisitor for Interpreter {
    type Result = Result<(), RuntimeError>;

    fn visit_expression(&mut self, expression: &crate::ast::Expression) -> Self::Result {
        expression.expression.accept(self).map(|_| {})
    }

    fn visit_print(&mut self, print: &crate::ast::Print) -> Self::Result {
        let value = print.expression.accept(self)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_var(&mut self, var: &crate::ast::Var) -> Self::Result {
        let value = match &var.initializer {
            Some(expr) => expr.accept(self)?,
            None => LoxValue::Nil,
        };

        match var.name.kind() {
            TokenKind::Identifier(id) => {
                self.environment.define(id.clone(), value);
                Ok(())
            }
            _ => Err(RuntimeError::new(
                var.name.clone(),
                "Expected identifier".to_string(),
            )),
        }
    }
}
