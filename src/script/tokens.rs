use core::fmt;

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {

  // Single-character tokens.
  LeftParen,  RightParen,
  LeftBrace,  RightBrace,
  Comma,      Dot,        Minus, Plus,
  Semicolon,  Slash,      Star,

  // One or two character tokens.
  Bang,    BangEqual,
  Equal,   EqualEqual,
  Greater, GreaterEqual,
  Less,    LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And,   Class,  Else,  False,
  For,   Fun,    If,    Nil,   Or,
  Print, Return, Super, This,
  True,  Var,    While,

  Error, EOF
}


#[derive(Eq, Debug, Clone)]
pub struct Token {
    pub type_:  TokenType,
    pub lexeme: String,
    pub line:   usize,
    pub col:    usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, line: usize, col: usize) -> Self {
        Self {
            type_,
            lexeme: String::from(lexeme),
            line,
            col,
        }
    }

    pub fn new_true() -> Self {
        Self::new(
            TokenType::True,
            "true",
            0,
            0,
        )
    }

    pub fn new_false() -> Self {
        Self::new(
            TokenType::False,
            "false",
            0,
            0,
        )
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {} {}", self.type_, self.lexeme, self.line, self.col)
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
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
           self.type_  == other.type_
        && self.lexeme == other.lexeme
    }
}
