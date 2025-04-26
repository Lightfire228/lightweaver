#![allow(unused)]

use TokenType::*;

use crate::{multi_line, script::{ast::{Connection, ExpressionStmt, Instantiation, VarDecl, Variable}, tokens::TokenType}};

use super::{ast::{Assign, Ast}, tokens::Token};

pub struct Example {
    pub source: String,
    pub tokens: Vec<Token>,
    pub ast:    Ast,
}


pub fn get_example_001() -> Example {
    Example { 
        source: multi_line!(
            "let a = Rect {};",
            "let b = Rect {};",
            "a -> b;",
        ), 
        tokens: vec![
            Token::new(LetToken,        "let",  1),
            Token::new(Identifier,      "a",    1),
            Token::new(Equals,          "=",    1),
            Token::new(RectToken,       "Rect", 1),
            Token::new(LeftCurly,       "{",    1),
            Token::new(RightCurly,      "}",    1),
            Token::new(SemiColon,       ";",    1),
            Token::new(LetToken,        "let",  2),
            Token::new(Identifier,      "b",    2),
            Token::new(Equals,          "=",    2),
            Token::new(RectToken,       "Rect", 2),
            Token::new(LeftCurly,       "{",    2),
            Token::new(RightCurly,      "}",    2),
            Token::new(SemiColon,       ";",    2),
            Token::new(Identifier,      "a",    3),
            Token::new(RightThinArrow,  "->",   3),
            Token::new(Identifier,      "b",    3),
            Token::new(SemiColon,       ";",    3),
            Token::new(EOFToken,        "",     3),
        ],
        ast: Ast {
            stmts: vec![
                VarDecl::new(
                    Token::new(Identifier, "a", 1),
                    Some(Instantiation::new(
                        Token::new(RectToken, "Rect", 1)
                    ))
                ),
                VarDecl::new(
                    Token::new(Identifier, "b", 2),
                    Some(Instantiation::new(
                        Token::new(RectToken, "Rect", 2)
                    ))
                ),
                ExpressionStmt::new(
                    Connection::new(
                        Variable::new(Token::new(Identifier,     "a",  3)),
                        Token              ::new(RightThinArrow, "->", 3),
                        Variable::new(Token::new(Identifier,     "b",  3)),
                    )
                ),
            ]
        }
    }
}

pub fn get_example_002() -> Example {
    Example {
        source: "a -> b;".to_owned(),
        tokens: vec![
            Token::new(Identifier,     "a",  1),
            Token::new(RightThinArrow, "->", 1),
            Token::new(Identifier,     "b",  1),
            Token::new(SemiColon,      ";",  1),
            Token::new(EOFToken,       "",   1),
        ],
        ast: Ast {
            stmts: vec![
                ExpressionStmt::new(
                    Connection::new(
                        Variable::new(Token::new(Identifier,     "a",  1)),
                        Token              ::new(RightThinArrow, "->", 1),
                        Variable::new(Token::new(Identifier,     "b",  1)),
                    )
                ),
            ]
        },
    }
}

pub fn get_example_003() -> Example {
    Example {
        source: "let a = Rect {};".to_owned(),
        tokens: vec![
            Token::new(LetToken,       "let",  1),
            Token::new(Identifier,     "a",    1),
            Token::new(Equals,         "=",    1),
            Token::new(RectToken,      "Rect", 1),
            Token::new(LeftCurly,      "}",    1),
            Token::new(RightCurly,     "{",    1),
            Token::new(SemiColon,      ";",    1),
            Token::new(EOFToken,       "",     1),
        ],
        ast: Ast {
            stmts: vec![
                VarDecl::new(
                    Token::new(Identifier, "a", 1),
                    Some(Instantiation::new(
                        Token::new(RectToken, "Rect", 1)
                    ))
                ),
            ]
        },
    }
}