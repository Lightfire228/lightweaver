use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grouping {
    pub expr: Box<Expr>
}


impl Grouping {
    pub fn new(
        expr: Expr,
    ) -> Expr {
        Expr::Grouping(Self {
            expr: Box::new(expr)
        })
    }
}

impl AstNode for Grouping {
    fn display(&self, _: DisplayArgs) {
        println!("Grouping")
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![self.expr.as_ast()]
    }
}
