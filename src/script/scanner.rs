
use super::tokens::{Token, TokenType::{self, *}};
use std::{collections::HashMap, string::String};

type Keywords   = HashMap<String, TokenType>;
type ScanResult = Result<Vec<Token>, Vec<ScannerError>>;

pub struct Scanner {

    start:   usize, // start of the current lexeme
    current: usize, // current character
    line:    usize,
    col:     usize,

    source: Vec<char>,
    tokens: Vec<Token>,
    errors: Vec<ScannerError>,

    keywords: Keywords
}

#[derive(Debug)]
pub enum ScannerErrorType {
    UnknownCharacter,
    UnknownOperator,
    UnknownEscapeSequence,
    UnterminatedString,
}

#[derive(Debug)]
pub struct ScannerError {
    pub _line:  usize,
    pub _col:   usize,
    pub _msg:   String,
    pub _type_: ScannerErrorType
}

impl Scanner {
    fn new(source: &str) -> Self {

        let bytes = source.chars().collect();

        Self {
            start:   0,
            current: 0,
            line:    1,
            col:     0,

            source: bytes,
            tokens: Vec::new(),
            errors: Vec::new(),

            keywords: get_keywords()
        }
    }

    pub fn scan_tokens(source: &str) -> ScanResult {
        let mut scanner = Scanner::new(source);

        while !scanner.is_eof() {
            scanner.start = scanner.current;

            scanner.scan_token();
        }

        scanner.finalize();

        if scanner.errors.len() == 0 {
            Ok(scanner.tokens)
        }
        else {
            Err(scanner.errors)
        }
    }

    // #region Tokenizing functions

    fn scan_token(&mut self) {
        let ch     = self.advance();
        self.col  += 1;

        match ch {

            ' '  => (),
            '\r' => (),
            '\t' => (),

            '='  => self.add_token("=", Equals),
            ':'  => self.add_token(":", Colon),
            ';'  => self.add_token(";", SemiColon),
            '{'  => self.add_token("{", LeftCurly),
            '}'  => self.add_token("}", RightCurly),
            '"'  => self.scan_string(),

            '-' => {
                if self.match_('>') {
                    self.advance();
                    self.add_token("->", RightThinArrow);
                }
                else {
                    self.error(format!("Unknown operator '-'"), ScannerErrorType::UnknownOperator);
                }
            }

            '\n' => {
                self.line += 1;
                self.col   = 0;
            }

            _ => {

                if is_digit(ch) {
                    self.scan_number();
                }
                else if is_alpha(ch) {
                    self.scan_identifier();
                }
                else {
                    self.error(format!("Unknown character '{}'", format_ch(ch)), ScannerErrorType::UnknownCharacter);
                }

            },

        }
    }

    fn scan_number(&mut self) {
        
        while is_alpha(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek()) {
            // consume the '.'
            self.advance();

            while is_alpha(self.peek()) {
                self.advance();
            }
        }

        self.add_token(&self.get_lexeme(), Number);
    }

    fn scan_string(&mut self) {

        let mut bytes: Vec<char> = vec![];

        while self.peek() != '"' && !self.is_eof() {

            if self.peek() == '\n' {
                self.line += 1;
            }

            else if self.peek() == '\\' && self.has_next() {
                self.advance();
                
                bytes.push(self.scan_escape());
            }

            bytes.push(self.advance());
        }

        if self.is_eof() {
            self.error(format!("Unterminated string"), ScannerErrorType::UnterminatedString);
        }

        // the closing "
        self.advance();

        self.add_token(&chars_to_str(&bytes), TokenType::StringToken);

    }

    fn scan_escape(&mut self) -> char {
        let ch = self.advance();

        match ch {
            '\"' |
            '\'' |
            '\\' => ch,
            'n'  => '\n',
            'r'  => '\r',
            't'  => '\t',

            _   => {
                self.error(format!("Unknown escape sequence '\\{}'", format_ch(ch)), ScannerErrorType::UnknownEscapeSequence);
                '\0'
            }
        }
    }

    fn scan_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let lexeme = self.get_lexeme();

        let type_ = self.keywords.get(&lexeme);
        let type_ = type_.unwrap_or(&Identifier);

        self.add_token(&lexeme, type_.clone());
    }

    // #endregion
    // #region scanner functions

    fn is_eof(&self) -> bool {
        !self.has_next()
    }

    fn has_next(&self) -> bool {
        self.current < self.source.len()
    }

    fn add_token(&mut self, lexeme: &str, type_: TokenType) {
        self.tokens.push(Token::new(type_, &lexeme, self.line));
    }

    fn advance(&mut self) -> char {
        let i = self.current;
        self.current += 1;

        self.source[i]
    }

    fn seek(&self, index: usize) -> char {
        if self.current + index >= self.source.len() {
            return '\0';
        }

        let i = self.current + index;
        self.source[i]
    }

    fn peek(&self) -> char {
        self.seek(0)
    }

    fn _peek_next(&self) -> char {
        self.seek(1)
    }

    fn match_(&self, ch: char) -> bool {
        ch == self.peek()
    }

    fn _match_number(&self) -> bool {
        let ch = self.peek();

        is_digit(ch)
    }

    fn get_lexeme(&self) -> String {
        chars_to_str(&self.source[self.start..self.current])
    }
    
    fn finalize(&mut self) {
        self.tokens.push(Token::new(EOFToken, "", self.line));
    }

    fn error(&mut self, msg: String, err_type: ScannerErrorType) {
        self.errors.push(ScannerError {
            _line:  self.line,
            _col:   self.col,
            _msg:   msg,
            _type_: err_type,
        });
    }

    // #endregion
    
}


fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_alpha(ch: char) -> bool {
       ('a' <= ch && ch <= 'z')
    || ('A' <= ch && ch <= 'Z')
    ||  ch == '_'
    
}

fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn chars_to_str(chars: &[char]) -> String {
    chars.iter().cloned().collect::<String>()
}

pub fn get_keywords() -> HashMap<String, TokenType> {
    let mut dict = HashMap::new();

    let mut add = |k: &str, v: TokenType| {
        dict.insert(String::from(k), v);
    };

    add("let",  LetToken);
    add("Rect", RectToken);

    dict
}

fn format_ch(ch: char) -> String {
    if ch < ' ' {
        format!("\\0x{:X}", ch as u8)
    }
    else {
        return String::from(ch)
    }
}

#[cfg(test)]
mod test {
    use crate::{multi_line, script::scanner::Scanner};
    use super::super::tokens::{Token, TokenType::*};

    
    #[test]
    fn base() {
        let str = multi_line!(
            "let a = Rect {};",
            "let b = Rect {};",
            "a -> b;",
        );

        let tokens = Scanner::scan_tokens(&str);

        match tokens {
            Err(_)     => assert!(false),
            Ok(tokens) => {
                assert_eq!(tokens, vec![
                    Token::new(LetToken,       "let",  1),
                    Token::new(Identifier,     "a",    1),
                    Token::new(Equals,         "=",    1),
                    Token::new(RectToken,      "Rect", 1),
                    Token::new(LeftCurly,      "{",    1),
                    Token::new(RightCurly,     "}",    1),
                    Token::new(SemiColon,      ";",    1),
                    
                    Token::new(LetToken,       "let",  2),
                    Token::new(Identifier,     "b",    2),
                    Token::new(Equals,         "=",    2),
                    Token::new(RectToken,      "Rect", 2),
                    Token::new(LeftCurly,      "{",    2),
                    Token::new(RightCurly,     "}",    2),
                    Token::new(SemiColon,      ";",    2),

                    Token::new(Identifier,     "a",    3),
                    Token::new(RightThinArrow, "->",   3),
                    Token::new(Identifier,     "b",    3),
                    Token::new(SemiColon,      ";",    3),

                    Token::new(EOFToken,       "",     3),
                ])
            }
        }
    }
}