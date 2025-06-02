
#![allow(dead_code)]

use crate::{multi_line, script::tokens::TokenType};

use super::tokens::Token;

type Tt = TokenType;

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
            Token::new(Tt::Class,      "class",        1, 0,),
            Token::new(Tt::Identifier, "Circle",       1, 0,),
            Token::new(Tt::LeftBrace,  "{",            1, 0,),
            Token::new(Tt::Identifier, "init",         2, 0,),
            Token::new(Tt::LeftParen,  "(",            2, 0,),
            Token::new(Tt::Identifier, "radius",       2, 0,),
            Token::new(Tt::RightParen, ")",            2, 0,),
            Token::new(Tt::LeftBrace,  "{",            2, 0,),
            Token::new(Tt::This,       "this",         3, 0,),
            Token::new(Tt::Dot,        ".",            3, 0,),
            Token::new(Tt::Identifier, "radius",       3, 0,),
            Token::new(Tt::Equal,      "=",            3, 0,),
            Token::new(Tt::Identifier, "radius",       3, 0,),
            Token::new(Tt::Semicolon,  ";",            3, 0,),
            Token::new(Tt::RightBrace, "}",            4, 0,),
            Token::new(Tt::Identifier, "area",         6, 0,),
            Token::new(Tt::LeftParen,  "(",            6, 0,),
            Token::new(Tt::RightParen, ")",            6, 0,),
            Token::new(Tt::LeftBrace,  "{",            6, 0,),
            Token::new(Tt::Return,     "return",       7, 0,),
            Token::new(Tt::Number,     "3.141592653",  7, 0,),
            Token::new(Tt::Star,       "*",            7, 0,),
            Token::new(Tt::This,       "this",         7, 0,),
            Token::new(Tt::Dot,        ".",            7, 0,),
            Token::new(Tt::Identifier, "radius",       7, 0,),
            Token::new(Tt::Star,       "*",            7, 0,),
            Token::new(Tt::This,       "this",         7, 0,),
            Token::new(Tt::Dot,        ".",            7, 0,),
            Token::new(Tt::Identifier, "radius",       7, 0,),
            Token::new(Tt::Semicolon,  ";",            7, 0,),
            Token::new(Tt::RightBrace, "}",            8, 0,),
            Token::new(Tt::RightBrace, "}",            9, 0,),
            Token::new(Tt::Var,        "var",         11, 0,),
            Token::new(Tt::Identifier, "circle",      11, 0,),
            Token::new(Tt::Equal,      "=",           11, 0,),
            Token::new(Tt::Identifier, "Circle",      11, 0,),
            Token::new(Tt::LeftParen,  "(",           11, 0,),
            Token::new(Tt::Number,     "4",           11, 0,),
            Token::new(Tt::RightParen, ")",           11, 0,),
            Token::new(Tt::Semicolon,  ";",           11, 0,),
            Token::new(Tt::Print,      "print",       12, 0,),
            Token::new(Tt::Identifier, "circle",      12, 0,),
            Token::new(Tt::Dot,        ".",           12, 0,),
            Token::new(Tt::Identifier, "area",        12, 0,),
            Token::new(Tt::Semicolon,  ";",           12, 0,),
            Token::new(Tt::EOF,        "",            12, 0,),
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
