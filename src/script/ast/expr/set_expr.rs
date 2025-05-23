use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Set {
    pub target: Box<Expr>,
    pub name:   Token,
    pub value:  Box<Expr>
}

impl Set {
    pub fn new(target: Expr, name: Token, value: Expr) -> Expr {
        Expr::Set(Self {
            target: Box::new(target),
            name,
            value: Box::new(value),
        })
    }
}

impl AstNode for Set {
    fn display(&self, _: DisplayArgs) {
        println!("Set")
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![
            self.target.as_ast(),
            self.value .as_ast(),
        ]
    }
}
