use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Binary {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
}


impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Self {
            left:     Box::new(left),
            operator,
            right:    Box::new(right),
        })
    }
}

impl AstNode for Binary {
    fn display(&self, _: DisplayArgs) {
        println!("Binary op ({})", self.operator.lexeme)
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![
            self.left .as_ast(),
            self.right.as_ast(),
        ]
    }
}
