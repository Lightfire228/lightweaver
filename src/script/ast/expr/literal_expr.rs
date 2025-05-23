use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
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
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Literal ({})", self.value.lexeme);

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![]
    }
}
