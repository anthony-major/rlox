use crate::token::Token;

pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Literal(Literal),
    Grouping(Grouping),
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
        Self { value: value }
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

pub trait ExprVisitor {
    type Result;

    fn visit_unary(&self, unary: &Unary) -> Self::Result;
    fn visit_binary(&self, binary: &Binary) -> Self::Result;
    fn visit_literal(&self, literal: &Literal) -> Self::Result;
    fn visit_grouping(&self, grouping: &Grouping) -> Self::Result;
}

pub trait ExprAccept {
    fn accept<V: ExprVisitor>(&self, visitor: &V) -> V::Result;
}

impl ExprAccept for Expr {
    fn accept<V: ExprVisitor>(&self, visitor: &V) -> V::Result {
        match self {
            Self::Unary(x) => visitor.visit_unary(x),
            Self::Binary(x) => visitor.visit_binary(x),
            Self::Literal(x) => visitor.visit_literal(x),
            Self::Grouping(x) => visitor.visit_grouping(x),
        }
    }
}
pub enum Stmt {
    Expression(Expression),
    Print(Print),
}

pub struct Expression {
    pub expression: Expr,
}

impl Expression {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: expression,
        }
    }
}

pub struct Print {
    pub expression: Expr,
}

impl Print {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: expression,
        }
    }
}

pub trait StmtVisitor {
    type Result;

    fn visit_expression(&self, expression: &Expression) -> Self::Result;
    fn visit_print(&self, print: &Print) -> Self::Result;
}

pub trait StmtAccept {
    fn accept<V: StmtVisitor>(&self, visitor: &V) -> V::Result;
}

impl StmtAccept for Stmt {
    fn accept<V: StmtVisitor>(&self, visitor: &V) -> V::Result {
        match self {
            Self::Expression(x) => visitor.visit_expression(x),
            Self::Print(x) => visitor.visit_print(x),
        }
    }
}
