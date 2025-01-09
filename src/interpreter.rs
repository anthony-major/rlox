use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

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
    Function(Function),
    NativeFunction(NativeFunction),
    Class(Class),
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
            Self::Function(fun) => write!(f, "{:?}", fun),
            Self::NativeFunction(nfun) => write!(f, "{:?}", nfun),
            Self::Class(c) => write!(f, "{:?}", c),
        }
    }
}

trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct NativeFunction {
    function: Rc<dyn Fn() -> LoxValue>,
}

impl NativeFunction {
    pub fn new(function: Rc<dyn Fn() -> LoxValue>) -> Self {
        Self { function }
    }
}

impl LoxCallable for NativeFunction {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, Box<dyn Error>> {
        Ok((self.function)())
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, _other: &Self) -> bool {
        false
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Clone)]
pub struct Function {
    declaration: crate::ast::Function,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: crate::ast::Function, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl LoxCallable for Function {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, Box<dyn Error>> {
        let mut environment = Environment::new(self.closure.clone());

        for (i, parameter) in self.declaration.params.iter().enumerate() {
            match parameter.kind() {
                TokenKind::Identifier(id) => environment.define(id.clone(), arguments[i].clone()),
                _ => {
                    return Err(Box::new(RuntimeError::new(
                        parameter.clone(),
                        "Expect identifier".to_string(),
                    )))
                }
            }
        }

        let previous = interpreter.environment.clone();
        interpreter.environment = Rc::new(RefCell::new(environment));

        for statement in &self.declaration.body {
            match statement.accept(&mut *interpreter) {
                Ok(_) => {}
                Err(err) => {
                    interpreter.environment = previous;

                    if let Some(return_err) = err.downcast_ref::<ReturnError>() {
                        return Ok(return_err.value.clone());
                    }

                    return Err(err);
                }
            }
        }

        interpreter.environment = previous;
        Ok(LoxValue::Nil)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self { name }
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

#[derive(Debug)]
struct ReturnError {
    value: LoxValue,
}

impl ReturnError {
    pub fn new(value: LoxValue) -> Self {
        Self { value }
    }
}

impl Error for ReturnError {}

impl Display for ReturnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut globals = Environment::default();

        globals.define(
            "clock".to_string(),
            LoxValue::NativeFunction(NativeFunction::new(Rc::new(|| {
                LoxValue::Number(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64(),
                )
            }))),
        );

        let environment = Rc::new(RefCell::new(globals));

        Self {
            environment: environment.clone(),
            globals: environment.clone(),
            locals: HashMap::new(),
        }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), Box<dyn Error>> {
        for statement in statements {
            statement.accept(self)?;
        }

        Ok(())
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    fn look_up_variable(&mut self, name: &Token, expr: &Expr) -> Result<LoxValue, Box<dyn Error>> {
        let distance = self.locals.get(expr);

        match distance {
            Some(distance) => self.environment.borrow_mut().get_at(*distance, name),
            None => self.globals.borrow_mut().get(name),
        }
    }
}

fn evaluate_number_operands<F: Fn(f64, f64) -> LoxValue>(
    operator: Token,
    left: LoxValue,
    right: LoxValue,
    operation: F,
) -> Result<LoxValue, Box<dyn Error>> {
    match (left, right) {
        (LoxValue::Number(x), LoxValue::Number(y)) => Ok(operation(x, y)),
        _ => Err(Box::new(RuntimeError::new(
            operator,
            "Expected two numbers for binary operator".to_string(),
        ))),
    }
}

impl ExprVisitor for Interpreter {
    type Result = Result<LoxValue, Box<dyn Error>>;

    fn visit_literal(&mut self, literal: &crate::ast::Literal) -> Self::Result {
        match literal.value.kind() {
            TokenKind::Nil => Ok(LoxValue::Nil),
            TokenKind::True => Ok(LoxValue::Boolean(true)),
            TokenKind::False => Ok(LoxValue::Boolean(false)),
            TokenKind::Number(x) => Ok(LoxValue::Number(x.clone())),
            TokenKind::String(s) => Ok(LoxValue::String(s.clone())),
            _ => Err(Box::new(RuntimeError::new(
                literal.value.clone(),
                "Expected literal".to_string(),
            ))),
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
                _ => Err(Box::new(RuntimeError::new(
                    unary.operator.clone(),
                    "Expected number after unary operator".to_string(),
                ))),
            },
            TokenKind::Bang => Ok(LoxValue::Boolean(!right.is_truthy())),
            _ => Err(Box::new(RuntimeError::new(
                unary.operator.clone(),
                "Expected unary operator".to_string(),
            ))),
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
                _ => Err(Box::new(RuntimeError::new(
                    binary.operator.clone(),
                    "Expected two numbers or two strings".to_string(),
                ))),
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
            _ => Err(Box::new(RuntimeError::new(
                binary.operator.clone(),
                "Expected binary operator".to_string(),
            ))),
        }
    }

    fn visit_variable(&mut self, variable: &crate::ast::Variable) -> Self::Result {
        self.look_up_variable(&variable.name, &Expr::Variable(variable.clone()))
        // match self.environment.borrow_mut().get(&variable.name) {
        //     Ok(value) => Ok(value.clone()),
        //     Err(err) => Err(err),
        // }
    }

    fn visit_assign(&mut self, assign: &crate::ast::Assign) -> Self::Result {
        let value = assign.value.accept(self)?;

        let distance = self.locals.get(&Expr::Assign(assign.clone()));
        match distance {
            Some(distance) => {
                self.environment
                    .borrow_mut()
                    .assign_at(*distance, &assign.name, value)
            }
            None => self.globals.borrow_mut().assign(&assign.name, value),
        }
    }

    fn visit_logical(&mut self, logical: &crate::ast::Logical) -> Self::Result {
        let left = logical.left.accept(self)?;

        match logical.operator.kind() {
            TokenKind::Or => {
                if left.is_truthy() {
                    return Ok(left);
                }
            }
            TokenKind::And => {
                if !left.is_truthy() {
                    return Ok(left);
                }
            }
            _ => {
                return Err(Box::new(RuntimeError::new(
                    logical.operator.clone(),
                    "Expected logical operator".to_string(),
                )))
            }
        }

        logical.right.accept(self)
    }

    fn visit_call(&mut self, call: &crate::ast::Call) -> Self::Result {
        let callee = call.callee.accept(self)?;

        let mut arguments: Vec<LoxValue> = Vec::new();
        for argument in &call.arguments {
            arguments.push(argument.accept(self)?);
        }

        let function: Box<dyn LoxCallable> = match callee {
            LoxValue::NativeFunction(nfun) => Box::new(nfun),
            LoxValue::Function(fun) => Box::new(fun),
            _ => {
                return Err(Box::new(RuntimeError::new(
                    call.paren.clone(),
                    "Can only call functions and classes".to_string(),
                )))
            }
        };

        if arguments.len() != function.arity() {
            return Err(Box::new(RuntimeError::new(
                call.paren.clone(),
                format!(
                    "Expected {} arguments but got {}",
                    function.arity(),
                    arguments.len()
                ),
            )));
        }

        function.call(self, arguments)
    }
}

impl StmtVisitor for Interpreter {
    type Result = Result<(), Box<dyn Error>>;

    fn visit_block(&mut self, block: &crate::ast::Block) -> Self::Result {
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(Environment::new(previous.clone())));

        for statement in &block.statements {
            match statement.accept(self) {
                Ok(_) => {}
                Err(err) => {
                    self.environment = previous;
                    return Err(err);
                }
            }
        }

        self.environment = previous;
        Ok(())
    }

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
                self.environment.borrow_mut().define(id.clone(), value);
                Ok(())
            }
            _ => Err(Box::new(RuntimeError::new(
                var.name.clone(),
                "Expected identifier".to_string(),
            ))),
        }
    }

    fn visit_ifstmt(&mut self, ifstmt: &crate::ast::IfStmt) -> Self::Result {
        if ifstmt.condition.accept(self)?.is_truthy() {
            ifstmt.then_branch.accept(self)?;
        } else if let Some(stmt) = &ifstmt.else_branch {
            stmt.accept(self)?;
        }

        Ok(())
    }

    fn visit_whilestmt(&mut self, whilestmt: &crate::ast::WhileStmt) -> Self::Result {
        while whilestmt.condition.accept(self)?.is_truthy() {
            whilestmt.body.accept(self)?;
        }

        Ok(())
    }

    fn visit_function(&mut self, function: &crate::ast::Function) -> Self::Result {
        let fun = LoxValue::Function(Function::new(function.clone(), self.environment.clone()));

        match function.name.kind() {
            TokenKind::Identifier(id) => self.environment.borrow_mut().define(id.clone(), fun),
            _ => {
                return Err(Box::new(RuntimeError::new(
                    function.name.clone(),
                    "Expect identifier".to_string(),
                )))
            }
        }

        Ok(())
    }

    fn visit_returnstmt(&mut self, returnstmt: &crate::ast::ReturnStmt) -> Self::Result {
        let value = match &returnstmt.value {
            Some(expr) => expr.accept(self)?,
            None => LoxValue::Nil,
        };

        Err(Box::new(ReturnError::new(value)))
    }

    fn visit_class(&mut self, class: &crate::ast::Class) -> Self::Result {
        let name = match class.name.kind() {
            TokenKind::Identifier(id) => id.clone(),
            _ => {
                return Err(Box::new(RuntimeError::new(
                    class.name.clone(),
                    "Expected identifier".to_string(),
                )))
            }
        };

        self.environment
            .borrow_mut()
            .define(name.clone(), LoxValue::Nil);
        let klass = LoxValue::Class(Class::new(name.clone()));
        self.environment.borrow_mut().assign(&class.name, klass);

        Ok(())
    }
}
