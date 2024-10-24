use crate::ast::{Accept, Expr, Visitor};

#[derive(Default)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: Expr) {
        expr.accept(self);
    }
}

impl Visitor for AstPrinter {
    type Result = ();

    fn visit_unary(&self, unary: &crate::ast::Unary) -> Self::Result {
        print!("({} ", unary.operator.kind());
        unary.right.accept(self);
        print!(")");
    }

    fn visit_binary(&self, binary: &crate::ast::Binary) -> Self::Result {
        print!("({} ", binary.operator.kind());
        binary.left.accept(self);
        binary.right.accept(self);
        print!(")");
    }

    fn visit_literal(&self, literal: &crate::ast::Literal) -> Self::Result {
        print!("{}", literal.value.kind());
    }

    fn visit_grouping(&self, grouping: &crate::ast::Grouping) -> Self::Result {
        print!("(group ");
        grouping.expression.accept(self);
    }
}
