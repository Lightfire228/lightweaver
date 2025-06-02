use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::{Token, TokenType}};

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BinaryOperator {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
    pub type_:    BinaryOpType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BinaryOpType {
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperator {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Self {
            type_:    get_type(&operator),
            left:     Box::new(left),
            operator,
            right:    Box::new(right),
        })
    }
}

impl AstNode for BinaryOperator {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Binary op ({})", self.operator.lexeme);

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

fn get_type(token: &Token) -> BinaryOpType {
    type T = TokenType;
    match token.type_ {

        T::BangEqual    => BinaryOpType::NotEqual,
        T::EqualEqual   => BinaryOpType::Equal,
        T::Greater      => BinaryOpType::Greater,
        T::GreaterEqual => BinaryOpType::GreaterEqual,
        T::Less         => BinaryOpType::Less,
        T::LessEqual    => BinaryOpType::LessEqual,

        T::Plus         => BinaryOpType::Add,
        T::Minus        => BinaryOpType::Subtract,
        T::Star         => BinaryOpType::Multiply,
        T::Slash        => BinaryOpType::Divide,
        _                 => panic!("Unknown token type ({}) for binary operator", token.type_),
    }
}
