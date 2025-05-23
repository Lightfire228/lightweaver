use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Expr;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    pub name:  Token,
    pub value: Box::<Expr>,
}


impl Assign {
    pub fn new(name: Token, value: Expr) -> Expr {
        Expr::Assign(Self {
            name,
            value: Box::new(value),
        })
    }
}


impl AstNode for Assign {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Assign".to_owned(),
            fields:  Some(vec![
                "= ".to_owned()
            ]),
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![self.value.as_ast()]
    }
}
