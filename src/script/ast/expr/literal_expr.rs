use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    pub value: Token,
}

impl Literal {
    pub fn new(value: Token) -> Expr {
        Expr::Literal(Self {
            value,
        })
    }
}

impl AstNode for Literal {
    fn display(&self, _: DisplayArgs) {
        println!("Literal ({})", self.value.lexeme)
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![]
    }
}
