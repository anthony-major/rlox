use crate::token::Token;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Literal(Literal),
    Grouping(Grouping),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Self {
            operator: operator,
            right: right,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Self {
            left: left,
            operator: operator,
            right: right,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Literal {
    pub value: Token,
}

impl Literal {
    pub fn new(value: Token) -> Self {
        Self { value: value }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Self {
            expression: expression,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name: name }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Box<Expr>) -> Self {
        Self {
            name: name,
            value: value,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Logical {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Self {
            left: left,
            operator: operator,
            right: right,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Self {
        Self {
            callee: callee,
            paren: paren,
            arguments: arguments,
        }
    }
}

pub trait ExprVisitor {
    type Result;

    fn visit_unary(&mut self, unary: &Unary) -> Self::Result;
    fn visit_binary(&mut self, binary: &Binary) -> Self::Result;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Result;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Result;
    fn visit_variable(&mut self, variable: &Variable) -> Self::Result;
    fn visit_assign(&mut self, assign: &Assign) -> Self::Result;
    fn visit_logical(&mut self, logical: &Logical) -> Self::Result;
    fn visit_call(&mut self, call: &Call) -> Self::Result;
}

pub trait ExprAccept {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Result;
}

impl ExprAccept for Expr {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Result {
        match self {
            Self::Unary(x) => visitor.visit_unary(x),
            Self::Binary(x) => visitor.visit_binary(x),
            Self::Literal(x) => visitor.visit_literal(x),
            Self::Grouping(x) => visitor.visit_grouping(x),
            Self::Variable(x) => visitor.visit_variable(x),
            Self::Assign(x) => visitor.visit_assign(x),
            Self::Logical(x) => visitor.visit_logical(x),
            Self::Call(x) => visitor.visit_call(x),
        }
    }
}
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Stmt {
    Block(Block),
    Expression(Expression),
    Print(Print),
    Var(Var),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    Function(Function),
    ReturnStmt(ReturnStmt),
    Class(Class),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self {
            statements: statements,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Expression {
    pub expression: Box<Expr>,
}

impl Expression {
    pub fn new(expression: Box<Expr>) -> Self {
        Self {
            expression: expression,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Print {
    pub expression: Box<Expr>,
}

impl Print {
    pub fn new(expression: Box<Expr>) -> Self {
        Self {
            expression: expression,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Box<Expr>>) -> Self {
        Self {
            name: name,
            initializer: initializer,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl IfStmt {
    pub fn new(
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    ) -> Self {
        Self {
            condition: condition,
            then_branch: then_branch,
            else_branch: else_branch,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Box<Expr>, body: Box<Stmt>) -> Self {
        Self {
            condition: condition,
            body: body,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self {
            name: name,
            params: params,
            body: body,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Box<Expr>>,
}

impl ReturnStmt {
    pub fn new(keyword: Token, value: Option<Box<Expr>>) -> Self {
        Self {
            keyword: keyword,
            value: value,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Class {
    pub name: Token,
    pub methods: Vec<Function>,
}

impl Class {
    pub fn new(name: Token, methods: Vec<Function>) -> Self {
        Self {
            name: name,
            methods: methods,
        }
    }
}

pub trait StmtVisitor {
    type Result;

    fn visit_block(&mut self, block: &Block) -> Self::Result;
    fn visit_expression(&mut self, expression: &Expression) -> Self::Result;
    fn visit_print(&mut self, print: &Print) -> Self::Result;
    fn visit_var(&mut self, var: &Var) -> Self::Result;
    fn visit_ifstmt(&mut self, ifstmt: &IfStmt) -> Self::Result;
    fn visit_whilestmt(&mut self, whilestmt: &WhileStmt) -> Self::Result;
    fn visit_function(&mut self, function: &Function) -> Self::Result;
    fn visit_returnstmt(&mut self, returnstmt: &ReturnStmt) -> Self::Result;
    fn visit_class(&mut self, class: &Class) -> Self::Result;
}

pub trait StmtAccept {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Result;
}

impl StmtAccept for Stmt {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Result {
        match self {
            Self::Block(x) => visitor.visit_block(x),
            Self::Expression(x) => visitor.visit_expression(x),
            Self::Print(x) => visitor.visit_print(x),
            Self::Var(x) => visitor.visit_var(x),
            Self::IfStmt(x) => visitor.visit_ifstmt(x),
            Self::WhileStmt(x) => visitor.visit_whilestmt(x),
            Self::Function(x) => visitor.visit_function(x),
            Self::ReturnStmt(x) => visitor.visit_returnstmt(x),
            Self::Class(x) => visitor.visit_class(x),
        }
    }
}
