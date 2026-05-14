use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct This {
    pub keyword: Token,
}


impl This {
    pub fn new(
        keyword: Token,
    ) -> Expr {
        Expr::This(Self {
            keyword,
        })
    }
}

impl AstNode for This {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "This".to_owned(),
            labels:  None,
        }
    }



    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        vec![]
    }
}
