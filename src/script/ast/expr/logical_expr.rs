use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Logical {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Logical(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}


impl AstNode for Logical {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Logical op ({})", self.operator.lexeme);

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  Some(vec![
                "Left:  ".to_owned(),
                "Right: ".to_owned(),
            ]),
        }
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
