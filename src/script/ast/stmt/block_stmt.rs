use crate::script::ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs};
use super::*;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub stmts:  Box::<Vec<Stmt>>,
    pub locals: usize,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Stmt {
        Stmt::Block(Block {
            stmts:  Box::new(stmts),
            locals: 0,
        })
    }
}

impl AstNode for Block {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Block".to_owned(),
            labels:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        self.stmts.iter().map(Stmt::as_ast).collect()
    }
}
