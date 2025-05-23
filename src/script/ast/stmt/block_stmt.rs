use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}};
use super::*;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub stmts: Box::<Vec<Stmt>>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Stmt {
        Stmt::Block(Block {
            stmts: Box::new(stmts),
        })
    }
}

impl AstNode for Block {
    fn display(&self, _: DisplayArgs) {
        println!("Block")
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        self.stmts.iter().map(Stmt::as_ast).collect()
    }
}
