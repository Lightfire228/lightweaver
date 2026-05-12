use crate::script::ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rect {
}


impl Rect {
    pub fn new(
    )
        -> Expr
    {
        Expr::Rect(Self {

        })
    }
}

impl AstNode for Rect {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Rect decl".to_owned(),
            labels:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        // vec![self.expr.as_ast()]
        vec![]

    }
}
