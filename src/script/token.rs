
#[derive(Debug)]
#[allow(unused)] // TODO:
pub struct Token {
    pub type_:  TokenType,
    pub lexeme: String,
    pub line:   usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, line: usize) -> Token {
        Token {
            type_,
            lexeme: String::from(lexeme),
            line,
        }
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)] // TODO:
pub enum TokenType {

    // Single-character tokens
    LEFT_PAREN,      RIGHT_PAREN,
    LEFT_CURLY,      RIGHT_CURLY,
    LEFT_SQ_BRACKET, RIGHT_SQ_BRACKET,

    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // 1-2 character tokens
    BANG,    BANG_EQUAL,
    EQUAL,   EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS,    LESS_EQUAL,

    FAT_ARROW, THIN_ARROW,

    // Literals
    IDENTIFIER, STRING, INTEGER, DOUBLE,

    // Keywords
    // AND,   CLASS,  ELSE,  FALSE, FUN,  FOR, IF,    NIL, OR,
    // PRINT, RETURN, SUPER, THIS,  TRUE, VAR, WHILE,

    EOF
}
