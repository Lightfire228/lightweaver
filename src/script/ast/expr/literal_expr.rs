use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::{Token, TokenType}};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    pub value: Token,
    pub type_: LiteralType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LiteralType {
    Nil,
    Number,
    Bool,
    String,
}

impl Literal {
    pub fn new(value: Token) -> Expr {
        Expr::Literal(Self {
            type_: get_type(&value),
            value,
        })
    }
}

impl AstNode for Literal {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Literal ({})", self.value.lexeme);

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


fn get_type(token: &Token) -> LiteralType {
    use TokenType::*;
    match token.type_ {
        TokenNumber => LiteralType::Number,
        TokenTrue   => LiteralType::Bool,
        TokenFalse  => LiteralType::Bool,
        TokenString => LiteralType::String,
        TokenNil    => LiteralType::Nil,
        _           => panic!("Unknown token type ({}) for literal", token.type_),
    }
}
