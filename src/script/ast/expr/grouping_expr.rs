use crate::script::ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs};
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
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Grouping".to_owned(),
            labels:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![self.expr.as_ast()]
    }
}
