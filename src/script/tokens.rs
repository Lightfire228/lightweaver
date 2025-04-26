use core::fmt;

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // 1 character tokens
    Equals, Colon, SemiColon,
    LeftCurly, RightCurly,

    // 2 Character tokens
    RightThinArrow,

    // Literals
    Identifier, StringToken, Number,

    // Keywords
    LetToken, RectToken,

    EOFToken
}


#[derive(Eq, Debug, Clone)]
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

    pub fn _to_string(&self) -> String {
        format!("{} {} {}", self.type_, self.lexeme, self.line)
    }

    pub fn _strict_eq(&self, other: &Self) -> bool {
           self.type_  == other.type_
        && self.lexeme == other.lexeme
        && self.line   == other.line
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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
           self.type_  == other.type_ 
        && self.lexeme == other.lexeme 
    }
}