use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token, vm::chunk::StackIndex};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
    pub name:   Token,
    pub closed: bool,
    pub decl:   StackIndex,
}


impl Variable {
    pub fn new(name: Token) -> Variable {
        Self {
            name,
            closed: false,
            decl:   StackIndex(0),
        }
    }

    pub fn as_expr(self) -> Expr {
        Expr::Variable(self)
    }
}

impl AstNode for Variable {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Variable ({})", self.name.lexeme);

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
