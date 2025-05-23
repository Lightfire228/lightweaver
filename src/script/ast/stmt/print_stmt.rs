use crate::script::ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs};
use crate::script::ast::Expr;

use super::Stmt;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrintStmt {
    pub expr: Expr,
}

impl PrintStmt {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Print(Self {
            expr,
        })
    }
}

impl AstNode for PrintStmt {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Print Stmt".to_owned(),
            fields:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![self.expr.as_ast()]
    }
}
