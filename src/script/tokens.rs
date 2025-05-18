use core::fmt;

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {

  // Single-character tokens.
  TokenLeftParen,  TokenRightParen,
  TokenLeftBrace,  TokenRightBrace,
  TokenComma,      TokenDot,        TokenMinus, TokenPlus,
  TokenSemicolon,  TokenSlash,      TokenStar,

  // One or two character tokens.
  TokenBang,    TokenBangEqual,
  TokenEqual,   TokenEqualEqual,
  TokenGreater, TokenGreaterEqual,
  TokenLess,    TokenLessEqual,

  // Literals.
  TokenIdentifier, TokenString, TokenNumber,

  // Keywords.
  TokenAnd,   TokenClass,  TokenElse,  TokenFalse,
  TokenFor,   TokenFun,    TokenIf,    TokenNil,   TokenOr,
  TokenPrint, TokenReturn, TokenSuper, TokenThis,
  TokenTrue,  TokenVar,    TokenWhile,

  TokenError, TokenEOF
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
            TokenType::TokenTrue,
            "true",
            0,
            0,
        )
    }

    pub fn new_false() -> Self {
        Self::new(
            TokenType::TokenFalse,
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
