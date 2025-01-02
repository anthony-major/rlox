use std::{borrow::BorrowMut, collections::HashMap, rc::Rc};

use crate::{
    ast::{Expr, ExprAccept, ExprVisitor, StmtAccept, StmtVisitor},
    interpreter::Interpreter,
    lox::Lox,
    parser::ParserError,
    token::{Token, TokenKind},
};

pub struct Resolver {
    interpreter: Rc<Interpreter>,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Rc<Interpreter>) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        if let TokenKind::Identifier(id) = name.kind() {
            scope.insert(id.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        if let TokenKind::Identifier(id) = name.kind() {
            scope.insert(id.clone(), true);
        }
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if let TokenKind::Identifier(id) = name.kind() {
                if scope.contains_key(id) {
                    self.interpreter
                        .borrow_mut()
                        .resolve(expr, self.scopes.len() - 1 - i);
                    return;
                }
            }
        }
    }
}

impl ExprVisitor for Resolver {
    type Result = ();

    fn visit_variable(&mut self, variable: &crate::ast::Variable) -> Self::Result {
        if !self.scopes.is_empty() {
            if let TokenKind::Identifier(id) = variable.name.kind() {
                if let Some(x) = self.scopes.last().unwrap().get(id) {
                    if *x == false {
                        Lox::error(Box::new(ParserError::new(
                            variable.name.clone(),
                            "Can't read local variable in its own initializer".to_string(),
                        )));
                    }
                }
            }
        }

        self.resolve_local(&Expr::Variable(variable.clone()), &variable.name);
    }

    fn visit_assign(&mut self, assign: &crate::ast::Assign) -> Self::Result {
        assign.value.accept(self);
        self.resolve_local(&Expr::Assign(assign.clone()), &assign.name);
    }
}

impl StmtVisitor for Resolver {
    type Result = ();

    fn visit_block(&mut self, block: &crate::ast::Block) -> Self::Result {
        self.scopes.push(HashMap::new());

        for statement in &block.statements {
            statement.accept(self);
        }

        self.scopes.pop();
    }

    fn visit_var(&mut self, var: &crate::ast::Var) -> Self::Result {
        self.declare(&var.name);

        if let Some(initializer) = var.initializer {
            initializer.accept(self);
        }

        self.define(&var.name);
    }
}
