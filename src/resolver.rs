use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{Expr, ExprAccept, ExprVisitor, Stmt, StmtAccept, StmtVisitor},
    interpreter::Interpreter,
    lox::Lox,
    parser::ParserError,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionKind {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassKind {
    None,
    Class,
}

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionKind,
    current_class: ClassKind,
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionKind::None,
            current_class: ClassKind::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            statement.accept(self);
        }
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        if let TokenKind::Identifier(id) = name.kind() {
            if scope.contains_key(id) {
                Lox::error(Box::new(ParserError::new(
                    name.clone(),
                    "Already a variable with this name in this scope.".to_string(),
                )));
            }

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

    fn resolve_function(&mut self, function: &crate::ast::Function, kind: FunctionKind) {
        let enclosing_function = self.current_function.clone();
        self.current_function = kind;

        self.scopes.push(HashMap::new());

        for parameter in &function.params {
            self.declare(parameter);
            self.define(parameter);
        }

        for statement in &function.body {
            statement.accept(self);
        }

        self.scopes.pop();

        self.current_function = enclosing_function;
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

    fn visit_binary(&mut self, binary: &crate::ast::Binary) -> Self::Result {
        binary.left.accept(self);
        binary.right.accept(self);
    }

    fn visit_call(&mut self, call: &crate::ast::Call) -> Self::Result {
        call.callee.accept(self);

        for argument in &call.arguments {
            argument.accept(self);
        }
    }

    fn visit_grouping(&mut self, grouping: &crate::ast::Grouping) -> Self::Result {
        grouping.expression.accept(self);
    }

    fn visit_literal(&mut self, _literal: &crate::ast::Literal) -> Self::Result {}

    fn visit_logical(&mut self, logical: &crate::ast::Logical) -> Self::Result {
        logical.left.accept(self);
        logical.right.accept(self);
    }

    fn visit_unary(&mut self, unary: &crate::ast::Unary) -> Self::Result {
        unary.right.accept(self);
    }

    fn visit_get(&mut self, get: &crate::ast::Get) -> Self::Result {
        get.object.accept(self);
    }

    fn visit_set(&mut self, set: &crate::ast::Set) -> Self::Result {
        set.value.accept(self);
        set.object.accept(self);
    }

    fn visit_this(&mut self, this: &crate::ast::This) -> Self::Result {
        if self.current_class == ClassKind::None {
            Lox::error(Box::new(ParserError::new(
                this.keyword.clone(),
                "Can't use 'this' outside of a class.".to_string(),
            )));
            return;
        }

        self.resolve_local(
            &Expr::This(this.clone()),
            &Token::new(
                TokenKind::Identifier("this".to_string()),
                this.keyword.line().clone(),
            ),
        );
    }

    fn visit_superexpr(&mut self, superexpr: &crate::ast::SuperExpr) -> Self::Result {
        self.resolve_local(
            &Expr::SuperExpr(superexpr.clone()),
            &Token::new(
                TokenKind::Identifier("super".to_string()),
                superexpr.keyword.line().clone(),
            ),
        );
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

        if let Some(initializer) = &var.initializer {
            initializer.accept(self);
        }

        self.define(&var.name);
    }

    fn visit_function(&mut self, function: &crate::ast::Function) -> Self::Result {
        self.declare(&function.name);
        self.define(&function.name);

        self.resolve_function(function, FunctionKind::Function);
    }

    fn visit_expression(&mut self, expression: &crate::ast::Expression) -> Self::Result {
        expression.expression.accept(self);
    }

    fn visit_ifstmt(&mut self, ifstmt: &crate::ast::IfStmt) -> Self::Result {
        ifstmt.condition.accept(self);
        ifstmt.then_branch.accept(self);
        if let Some(branch) = &ifstmt.else_branch {
            branch.accept(self);
        }
    }

    fn visit_print(&mut self, print: &crate::ast::Print) -> Self::Result {
        print.expression.accept(self);
    }

    fn visit_returnstmt(&mut self, returnstmt: &crate::ast::ReturnStmt) -> Self::Result {
        if self.current_function == FunctionKind::None {
            Lox::error(Box::new(ParserError::new(
                returnstmt.keyword.clone(),
                "Can't return from top-level code.".to_string(),
            )))
        }

        if let Some(value) = &returnstmt.value {
            if self.current_function == FunctionKind::Initializer {
                Lox::error(Box::new(ParserError::new(
                    returnstmt.keyword.clone(),
                    "Can't return a value from an initializer.".to_string(),
                )));
            }
            value.accept(self);
        }
    }

    fn visit_whilestmt(&mut self, whilestmt: &crate::ast::WhileStmt) -> Self::Result {
        whilestmt.condition.accept(self);
        whilestmt.body.accept(self);
    }

    fn visit_class(&mut self, class: &crate::ast::Class) -> Self::Result {
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassKind::Class;

        self.declare(&class.name);
        self.define(&class.name);

        if let Some(superclass) = &class.superclass {
            let class_name = match class.name.kind() {
                TokenKind::Identifier(id) => id.clone(),
                _ => "".to_string(),
            };
            match *superclass.clone() {
                Expr::Variable(var) => match var.name.kind() {
                    TokenKind::Identifier(id) => {
                        if class_name == id.clone() {
                            Lox::error(Box::new(ParserError::new(
                                var.name.clone(),
                                "A class can't inherit from itself.".to_string(),
                            )));
                        }
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }

            superclass.accept(self);

            self.scopes.push(HashMap::new());
            self.scopes
                .last_mut()
                .unwrap()
                .insert("super".to_string(), true);
        }

        self.scopes.push(HashMap::new());
        self.scopes
            .last_mut()
            .unwrap()
            .insert("this".to_string(), true);

        for method in &class.methods {
            let kind = match method.name.kind() {
                TokenKind::Identifier(id) => {
                    if id == "init" {
                        FunctionKind::Initializer
                    } else {
                        FunctionKind::Method
                    }
                }
                _ => FunctionKind::Method,
            };
            self.resolve_function(method, kind);
        }

        self.scopes.pop();

        if class.superclass.is_some() {
            self.scopes.pop();
        }

        self.current_class = enclosing_class;
    }
}
