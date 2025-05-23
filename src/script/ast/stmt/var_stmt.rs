use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use crate::script::{ast::Expr};

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarStmt {
    pub name:        Token,
    pub initializer: Option<Expr>,
}

impl VarStmt {
    pub fn new(name: Token, initializer: Option<Expr>) -> Stmt {
        Stmt::Var(VarStmt {
            name,
            initializer,
        })
    }
}

impl AstNode for VarStmt {
    fn display(&self, _: DisplayArgs) {
        println!("Variable ({})", self.name.lexeme)
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        match &self.initializer {
            Some(init) => vec![init.as_ast()],
            None       => vec![],
        }
    }
}
