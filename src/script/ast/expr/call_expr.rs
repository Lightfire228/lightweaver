use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren:  Token,
    pub args:   Box<Vec<Expr>>,
}


impl Call {
    pub fn new(
        callee: Expr,
        paren:  Token,
        args:   Vec<Expr>,
    ) -> Expr {
        Expr::Call(Self {
            callee: Box::new(callee),
            paren,
            args:   Box::new(args),
        })
    }
}

impl AstNode for Call {
    fn display(&self, args: DisplayArgs) -> AstDisplay {

        let mut fields = vec![
            "Callee: ".to_owned(),
        ];

        fields.extend(self.args.iter().map(|_| "Arg:    ".to_owned()));

        AstDisplay {
            depth:   args.depth,
            primary: "Call".to_owned(),
            fields:  Some(fields),
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        let mut results = vec![];

        results.push(self.callee.as_ast());
        results.extend(self.args.iter().map(Expr::as_ast));

        results
    }
}
