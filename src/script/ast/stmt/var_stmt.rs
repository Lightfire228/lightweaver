use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
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
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Variable ({})", self.name.lexeme);

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  self.initializer.as_ref().map(|_| vec!["Initializer: ".to_owned()])
        }
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
