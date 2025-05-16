
#![allow(dead_code)]
use TokenType::*;

use crate::{multi_line, script::tokens::TokenType};

use super::tokens::Token;

pub struct Example {
    pub source: String,
    pub tokens: Vec<Token>,
    // pub ast:    Ast,
}

pub fn get_example_001() -> Example {
    Example {
        source: multi_line!(
            "class Circle {",
            "  init(radius) {",
            "    this.radius = radius;",
            "  }",
            "",
            "  area() {",
            "    return 3.141592653 * this.radius * this.radius;",
            "  }",
            "}",
            "",
            "var circle = Circle(4);",
            "print circle.area; // Prints roughly \"50.2655\".",
        ),
        tokens: vec![
            Token::new(TokenClass,      "class",        1, 0,),
            Token::new(TokenIdentifier, "Circle",       1, 0,),
            Token::new(TokenLeftBrace,  "{",            1, 0,),
            Token::new(TokenIdentifier, "init",         2, 0,),
            Token::new(TokenLeftParen,  "(",            2, 0,),
            Token::new(TokenIdentifier, "radius",       2, 0,),
            Token::new(TokenRightParen, ")",            2, 0,),
            Token::new(TokenLeftBrace,  "{",            2, 0,),
            Token::new(TokenThis,       "this",         3, 0,),
            Token::new(TokenDot,        ".",            3, 0,),
            Token::new(TokenIdentifier, "radius",       3, 0,),
            Token::new(TokenEqual,      "=",            3, 0,),
            Token::new(TokenIdentifier, "radius",       3, 0,),
            Token::new(TokenSemicolon,  ";",            3, 0,),
            Token::new(TokenRightBrace, "}",            4, 0,),
            Token::new(TokenIdentifier, "area",         6, 0,),
            Token::new(TokenLeftParen,  "(",            6, 0,),
            Token::new(TokenRightParen, ")",            6, 0,),
            Token::new(TokenLeftBrace,  "{",            6, 0,),
            Token::new(TokenReturn,     "return",       7, 0,),
            Token::new(TokenNumber,     "3.141592653",  7, 0,),
            Token::new(TokenStar,       "*",            7, 0,),
            Token::new(TokenThis,       "this",         7, 0,),
            Token::new(TokenDot,        ".",            7, 0,),
            Token::new(TokenIdentifier, "radius",       7, 0,),
            Token::new(TokenStar,       "*",            7, 0,),
            Token::new(TokenThis,       "this",         7, 0,),
            Token::new(TokenDot,        ".",            7, 0,),
            Token::new(TokenIdentifier, "radius",       7, 0,),
            Token::new(TokenSemicolon,  ";",            7, 0,),
            Token::new(TokenRightBrace, "}",            8, 0,),
            Token::new(TokenRightBrace, "}",            9, 0,),
            Token::new(TokenVar,        "var",         11, 0,),
            Token::new(TokenIdentifier, "circle",      11, 0,),
            Token::new(TokenEqual,      "=",           11, 0,),
            Token::new(TokenIdentifier, "Circle",      11, 0,),
            Token::new(TokenLeftParen,  "(",           11, 0,),
            Token::new(TokenNumber,     "4",           11, 0,),
            Token::new(TokenRightParen, ")",           11, 0,),
            Token::new(TokenSemicolon,  ";",           11, 0,),
            Token::new(TokenPrint,      "print",       12, 0,),
            Token::new(TokenIdentifier, "circle",      12, 0,),
            Token::new(TokenDot,        ".",           12, 0,),
            Token::new(TokenIdentifier, "area",        12, 0,),
            Token::new(TokenSemicolon,  ";",           12, 0,),
            Token::new(TokenEOF,        "",            12, 0,),
        ],
    }
}

// pub fn get_example_002() -> Example {
//     Example {
//         source: "a -> b;".to_owned(),
//         tokens: vec![
//             Token::new(Identifier,     "a",  1),
//             Token::new(RightThinArrow, "->", 1),
//             Token::new(Identifier,     "b",  1),
//             Token::new(SemiColon,      ";",  1),
//             Token::new(EOFToken,       "",   1),
//         ],
//         ast: Ast {
//             stmts: vec![
//                 ExpressionStmt::new(
//                     Connection::new(
//                         Variable::new(Token::new(Identifier,     "a",  1)),
//                         Token              ::new(RightThinArrow, "->", 1),
//                         Variable::new(Token::new(Identifier,     "b",  1)),
//                     )
//                 ),
//             ]
//         },
//     }
// }

// pub fn get_example_003() -> Example {
//     Example {
//         source: "let a = Rect {};".to_owned(),
//         tokens: vec![
//             Token::new(LetToken,       "let",  1),
//             Token::new(Identifier,     "a",    1),
//             Token::new(Equals,         "=",    1),
//             Token::new(RectToken,      "Rect", 1),
//             Token::new(LeftCurly,      "}",    1),
//             Token::new(RightCurly,     "{",    1),
//             Token::new(SemiColon,      ";",    1),
//             Token::new(EOFToken,       "",     1),
//         ],
//         ast: Ast {
//             stmts: vec![
//                 VarDecl::new(
//                     Token::new(Identifier, "a", 1),
//                     Some(Instantiation::new(
//                         Token::new(RectToken, "Rect", 1)
//                     ))
//                 ),
//             ]
//         },
//     }
// }
