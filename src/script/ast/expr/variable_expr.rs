use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
    pub name: Token,
}


impl Variable {
    pub fn new(name: Token) -> Expr {
        Expr::Variable(Self { name })
    }
}

impl AstNode for Variable {
    fn display(&self, _: DisplayArgs) {
        println!("Variable ({})", self.name.lexeme)
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![]
    }
}
