use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::{Token, TokenType}};

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UnaryOperator {
    pub operator: Token,
    pub right:    Box<Expr>,
    pub type_:    UnaryOpType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnaryOpType {
    LogicalNot,
    Negate,
}


impl UnaryOperator {
    pub fn new(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Self {
            type_: get_type(&operator),
            operator,
            right: Box::new(right),
        })
    }
}

impl AstNode for UnaryOperator {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Unary op ({})", self.operator.lexeme);

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  Some(vec![
                "Right: ".to_owned(),
            ]),
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![self.right.as_ast()]
    }
}


fn get_type(token: &Token) -> UnaryOpType {
    use TokenType::*;
    match token.type_ {
        TokenBang  => UnaryOpType::LogicalNot,
        TokenMinus => UnaryOpType::Negate,
        _          => panic!("Unknown token type ({}) for unary operator", token.type_),
    }
}
