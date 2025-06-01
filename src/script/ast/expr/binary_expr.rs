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
    use TokenType::*;
    match token.type_ {
        TokenPlus   => BinaryOpType::Add,
        TokenMinus  => BinaryOpType::Subtract,
        TokenStar   => BinaryOpType::Multiply,
        TokenSlash  => BinaryOpType::Divide,
        _           => panic!("Unknown token type ({}) for binary operator", token.type_),
    }
}
