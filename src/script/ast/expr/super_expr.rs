use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Super {
    pub keyword: Token,
    pub method:  Token,
}


impl Super {
    pub fn new(
        keyword: Token,
        method:  Token,

    ) -> Expr {
        Expr::Super(Self {
            keyword,
            method,
        })
    }
}

impl AstNode for Super {
    fn display(&self, _: DisplayArgs) {
        println!("Super")
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![]
    }
}
