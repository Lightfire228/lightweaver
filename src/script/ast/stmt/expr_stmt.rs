use crate::script::ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs};
use crate::script::ast::Expr;

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionStmt {
    pub expr: Expr,
}

impl ExpressionStmt {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Expression(Self {
            expr,
        })
    }
}

impl AstNode for ExpressionStmt {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Expr Stmt".to_owned(),
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
