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
    True,
    False,
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

    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        vec![]
    }
}


fn get_type(token: &Token) -> LiteralType {
    type T = TokenType;
    match token.type_ {
        T::Number => LiteralType::Number,
        T::True   => LiteralType::True,
        T::False  => LiteralType::False,
        T::String => LiteralType::String,
        T::Nil    => LiteralType::Nil,
        _           => panic!("Unknown token type ({}) for literal", token.type_),
    }
}
