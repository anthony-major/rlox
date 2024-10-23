use crate::token::Token;

pub enum Expr {
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Literal(Box<Literal>),
    Grouping(Box<Grouping>),
}

pub struct Unary {
    operator: Token,
    right: Box<Expr>,
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
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
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
    value: Token,
}

impl Literal {
    pub fn new(value: Token) -> Self {
        Self {
            value: value,
        }
    }
}

pub struct Grouping {
    expression: Box<Expr>,
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

pub trait AcceptExpr {
    fn accept<V: ExprVisitor>(&self, visitor: &V) -> V::Result;
}

impl AcceptExpr for Expr {
    fn accept<V: ExprVisitor>(&self, visitor: &V) -> V::Result {
        match self {
            Self::Unary(x) => visitor.visit_unary(x),
            Self::Binary(x) => visitor.visit_binary(x),
            Self::Literal(x) => visitor.visit_literal(x),
            Self::Grouping(x) => visitor.visit_grouping(x),
        }
    }
}
