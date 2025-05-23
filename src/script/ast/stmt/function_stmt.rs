use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Stmt;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionStmt {
    pub name:   Token,
    pub params: Vec<Token>,
    pub body:   Box<Vec<Stmt>>
}


impl FunctionStmt {

    pub fn new(
        name:   Token,
        params: Vec<Token>,
        body:   Vec<Stmt>,
    ) -> Self {
        Self {
            name,
            params,
            body: Box::new(body),
        }
    }
}

impl AstNode for FunctionStmt {
    fn display(&self, _: DisplayArgs) {
        println!("Function ({})", self.name.lexeme)
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        self.body.iter().map(Stmt::as_ast).collect()
    }
}
