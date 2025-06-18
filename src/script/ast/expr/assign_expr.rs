use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, Variable, WalkArgs}};

use super::Expr;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    pub target: Variable,
    pub value:  Box::<Expr>,
}


impl Assign {
    pub fn new(target: Variable, value: Expr) -> Expr {
        Expr::Assign(Self {
            target,
            value: Box::new(value),
        })
    }
}


impl AstNode for Assign {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Assign".to_owned(),
            labels:  Some(vec![
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
