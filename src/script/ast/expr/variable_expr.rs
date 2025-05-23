use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
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
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Variable ({})", self.name.lexeme);
        
        AstDisplay {
            depth:   args.depth,
            primary: msg,
            fields:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![]
    }
}
