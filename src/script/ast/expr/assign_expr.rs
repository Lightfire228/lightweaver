use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Expr;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    pub name:  Token,
    pub value: Box::<Expr>,
}


impl Assign {
    pub fn new(name: Token, value: Expr) -> Expr {
        Expr::Assign(Self {
            name,
            value: Box::new(value),
        })
    }
}


impl AstNode for Assign {
    fn display(&self, _: DisplayArgs) {
        println!("Assign")
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![self.value.as_ast()]
    }
}
