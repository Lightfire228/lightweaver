use core::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum TokenType {
    // 1 character tokens
    Equals, Colon,
    LeftCurly, RightCurly,

    // 2 Character tokens
    RightThinArrow,

    // Literals
    Identifier, String, Number,

    // Keywords
    Let, Rect,

    EOF
}


pub struct Token {
    pub type_:  TokenType,
    pub lexeme: String,
    pub line:   usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, line: usize) -> Self {
        Self {
            type_,
            lexeme: String::from(lexeme),
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.type_, self.lexeme, self.line)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.type_, self.lexeme, self.line)
    }
}
