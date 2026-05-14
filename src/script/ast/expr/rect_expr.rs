use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rect {
    pub params: Vec<(Token, Expr)>,
}


pub type RectParams = Vec<(Token, Expr)>;

impl Rect {
    pub fn new(
        params: RectParams,
    )
        -> Expr
    {
        Expr::Rect(Self {
            params,
        })
    }
}

impl AstNode for Rect {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Rect decl".to_owned(),
            labels:  None,
        }
    }



    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        // vec![self.expr.as_ast()]
        vec![]

    }
}
