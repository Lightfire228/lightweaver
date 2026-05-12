
use super::tokens::{Token, TokenType};
use std::{collections::HashMap, string::String};

type Keywords   = HashMap<String, TokenType>;
type ScanResult = Result<Vec<Token>, Vec<ScannerError>>;

type Se = ScannerErrorType;
type Tt = TokenType;

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


struct Scanner {

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
    UnterminatedString,
    UnexpectedCharacter(String),
}

#[derive(Debug)]
pub struct ScannerError {
    pub line:  usize,
    pub col:   usize,
    pub type_: ScannerErrorType
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

    // #region Tokenizing functions

    fn scan_token(&mut self) {

        self.skip_whitespace();

        if self.is_eof() {
            return;
        }

        self.start = self.current;
        let ch = self.advance();

        if is_alpha(ch) {
            self.parse_identifier();
            return;
        }
        if is_digit(ch) {
            self.parse_number();
            return;
        }

        match ch {

            '(' => self.add_token(Tt::LeftParen),
            ')' => self.add_token(Tt::RightParen),
            '{' => self.add_token(Tt::LeftBrace),
            '}' => self.add_token(Tt::RightBrace),
            ';' => self.add_token(Tt::Semicolon),
            ',' => self.add_token(Tt::Comma),
            '.' => self.add_token(Tt::Dot),
            '-' => self.add_token(Tt::Minus),
            '+' => self.add_token(Tt::Plus),
            '/' => self.add_token(Tt::Slash),
            '*' => self.add_token(Tt::Star),

            '!' => self.add_token_match('=', Tt::Bang,    Tt::BangEqual),
            '=' => self.add_token_match('=', Tt::Equal,   Tt::EqualEqual),
            '<' => self.add_token_match('=', Tt::Less,    Tt::LessEqual),
            '>' => self.add_token_match('=', Tt::Greater, Tt::GreaterEqual),

            '"' => self.parse_string(),

            _   => self.add_error(Se::UnexpectedCharacter(ch.to_string())),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let ch = self.peek();
            match ch {
                  ' '
                | '\r'
                | '\t' => {
                    self.advance();
                }

                '\n' => {
                    self.line();
                    self.advance();
                }

                '/' => {
                    // don't consume
                    if self.peek_next() != '/' {
                        return;
                    }

                    // Consume comment to the end of a line
                    while self.peek() != '\n' && !self.is_eof() {
                        self.advance();
                    }
                }

                // don't consume
                _ => return,
            }
        }
    }

    fn parse_string(&mut self) {

        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line();
            }
            self.advance();
        }

        if self.is_eof() {
            self.add_error(Se::UnterminatedString);
        }

        self.advance();
        self.add_string_token();
    }

    fn parse_number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        // look for a fractional part
        if self.peek() == '.' && is_digit(self.peek_next()) {

            // consume the dot
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(Tt::Number);
    }

    fn parse_identifier(&mut self) {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        self.add_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {

        let lexeme = self.get_lexeme();

        self.keywords.get(&lexeme).unwrap_or(&Tt::Identifier).to_owned()
    }

    // #endregion
    // #region scanner functions

    fn is_eof(&self) -> bool {
        !self.has_next()
    }

    fn has_next(&self) -> bool {
        self.current < self.source.len()
    }

    fn add_token(&mut self, type_: TokenType) {
        let lexeme = self.get_lexeme();
        self.tokens.push(Token::new(type_, &lexeme, self.line, self.col));

        self.start = self.current;
    }

    fn add_string_token(&mut self) {
        let lexeme = self.get_wrapped_lexeme(1);
        self.tokens.push(Token::new(Tt::String, &lexeme, self.line, self.col));

        self.start = self.current;
    }

    fn add_token_match(&mut self, ch: char, first_type: TokenType, second_type: TokenType) {
        if self.match_(ch) {
            self.advance();
            self.add_token(second_type);
        }
        else {
            self.add_token(first_type);
        }
    }

    fn advance(&mut self) -> char {
        let i = self.current;
        self.current += 1;
        self.col     += 1;

        self.source[i]
    }

    fn line(&mut self) {
        self.line += 1;
        self.col   = 0;
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

    fn peek_next(&self) -> char {
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

    fn get_wrapped_lexeme(&self, i: usize) -> String {
        chars_to_str(&self.source[self.start +i .. self.current -i])
    }

    fn finalize(&mut self) {
        self.tokens.push(Token::new(Tt::EOF, "", self.line, self.col));
    }

    fn add_error(&mut self, err_type: ScannerErrorType) {
        self.errors.push(ScannerError {
            line:  self.line,
            col:   self.col,
            type_: err_type,
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
    [
        ("and",     Tt::And),
        ("class",   Tt::Class),
        ("else",    Tt::Else),
        ("false",   Tt::False),
        ("for",     Tt::For),
        ("fun",     Tt::Fun),
        ("if",      Tt::If),
        ("nil",     Tt::Nil),
        ("or",      Tt::Or),
        ("print",   Tt::Print),
        ("Rect",    Tt::Rect),
        ("return",  Tt::Return),
        ("super",   Tt::Super),
        ("this",    Tt::This),
        ("true",    Tt::True),
        ("var",     Tt::Var),
        ("while",   Tt::While),
    ]
        .iter   ()
        .map    (|(s, t)| (String::from(*s), *t))
        .collect()
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
mod tests {
    use crate::script::test::get_example_001;

    use super::scan_tokens;



    #[test]
    fn base() {
        let example = get_example_001();

        let tokens = scan_tokens(&example.source);

        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(tokens, example.tokens);
    }

}
