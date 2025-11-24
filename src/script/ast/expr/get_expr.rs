use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Get {
    pub expr: Box<Expr>,
    pub name: Token,
}


impl Get {
    pub fn new(expr: Expr, name: Token) -> Expr {
        Expr::Get(Self {
            expr: Box::new(expr),
            name,
        })
    }
}

impl AstNode for Get {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Get {}", self.name.lexeme);

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        vec![self.expr.as_ast()]
    }
}
