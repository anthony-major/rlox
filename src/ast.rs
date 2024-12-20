use crate::token::Token;

pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Literal(Literal),
    Grouping(Grouping),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
}

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

pub struct Literal {
    pub value: Token,
}

impl Literal {
    pub fn new(value: Token) -> Self {
        Self {
            value: value,
        }
    }
}

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

pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self {
            name: name,
        }
    }
}

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

pub trait ExprVisitor {
    type Result;

    fn visit_unary(&mut self, unary: &Unary) -> Self::Result;
    fn visit_binary(&mut self, binary: &Binary) -> Self::Result;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Result;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Result;
    fn visit_variable(&mut self, variable: &Variable) -> Self::Result;
    fn visit_assign(&mut self, assign: &Assign) -> Self::Result;
    fn visit_logical(&mut self, logical: &Logical) -> Self::Result;
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
        }
    }
}
pub enum Stmt {
    Block(Block),
    Expression(Expression),
    Print(Print),
    Var(Var),
    IfStmt(IfStmt),
}

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

pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl IfStmt {
    pub fn new(condition: Box<Expr>, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Self {
        Self {
            condition: condition,
            then_branch: then_branch,
            else_branch: else_branch,
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
        }
    }
}
