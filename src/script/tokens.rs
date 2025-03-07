
#[derive(Clone, Copy)]
pub enum TokenType {
    // 1 character tokens
    Equals,
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
}