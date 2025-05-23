use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right:    Box<Expr>,
}


impl Unary {
    pub fn new(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Self {
            operator,
            right: Box::new(right),
        })
    }
}

impl AstNode for Unary {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Unary op ({})", self.operator.lexeme);
        
        AstDisplay {
            depth:   args.depth,
            primary: msg,
            fields:  Some(vec![
                "Right: ".to_owned(),
            ]),
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![self.right.as_ast()]
    }
}
