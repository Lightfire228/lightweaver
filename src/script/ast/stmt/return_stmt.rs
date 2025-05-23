use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use crate::script::{ast::Expr};

use super::Stmt;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value:   Option<Expr>,
}

impl ReturnStmt {
    pub fn new(keyword: Token, value: Option<Expr>) -> Stmt {
        Stmt::Return(Self {
            keyword,
            value,
        })
    }
}

impl AstNode for ReturnStmt {
    fn display(&self, _: DisplayArgs) {
        println!("Return Stmt")
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        match &self.value {
            Some(value) => vec![value.as_ast()],
            None        => vec![],
        }
    }
}
