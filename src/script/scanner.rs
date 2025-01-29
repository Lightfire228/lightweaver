
use crate::ScriptRuntime;

use super::token::{Token, TokenType::{self, *}};

pub struct Scanner <'a> {

    runtime: &'a mut ScriptRuntime,

    start:   usize, // start of the current lexeme
    current: usize, // current character
    line:    usize,

    source: Vec<char>,
    tokens: Vec<Token>,
}

impl <'a> Scanner <'a> {
    pub fn new(runtime: &'a mut ScriptRuntime, source: &str) -> Scanner <'a> {

        // convert the file from UTF-8 variable width encoding to an array of 16 bit words 
        // to allow the scanner to index into the string
        let bytes = source.chars().collect();

        Scanner {
            runtime,

            start:   0,
            current: 0,
            line:    1,

            source: bytes,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(runtime: &'a mut ScriptRuntime, source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(runtime, source);

        while !scanner.is_eof() {
            scanner.start = scanner.current;

            scanner.scan_token();
        }

        scanner.finalize();

        scanner.tokens
    }

    // --- tokenizing functions

    fn scan_token(&mut self) {
        let ch     = self.advance();
        let as_str = ch.to_string();

        match ch {

            // Ignore whitespace
            ' '  => {},
            '\r' => {},
            '\t' => {},

            '\n' => self.line += 1,

            '('  => self.add_token(LEFT_PAREN),
            ')'  => self.add_token(RIGHT_PAREN),
            '{'  => self.add_token(LEFT_CURLY),
            '}'  => self.add_token(RIGHT_CURLY),
            '['  => self.add_token(LEFT_SQ_BRACKET),
            ']'  => self.add_token(RIGHT_SQ_BRACKET),
            ','  => self.add_token(COMMA),
            '.'  => self.add_token(DOT),
            '-'  => self.add_token(MINUS),
            '+'  => self.add_token(PLUS),
            ';'  => self.add_token(SEMICOLON),
            '*'  => self.add_token(STAR),

            '!'  => self.add_token(if self.match_('=') {BANG_EQUAL}    else {EQUAL}),
            '='  => self.add_token(if self.match_('=') {EQUAL_EQUAL}   else {EQUAL}),
            '<'  => self.add_token(if self.match_('=') {LESS_EQUAL}    else {LESS}),
            '>'  => self.add_token(if self.match_('=') {GREATER_EQUAL} else {GREATER}),

            '/'  => {
                if self.match_('/') {

                    // consume a comment to the end of the line
                    while self.peek() != '\n' && !self.is_eof() {
                        self.advance();
                    }
                }
                else {
                    self.add_token(SLASH);
                }
            }

            '"'  => self.handle_string(),
            '\'' => self.handle_string(),

            _ => {

                if is_digit(ch) {
                    self.handle_number();
                }
                else if is_alpha(ch) {
                    self.handle_identifier()
                }
                else {
                    self.runtime.error_tokenizing(self.line, "Unexpected character.");
                }
            },
        }
    }

    fn handle_string(&mut self) {
        // TODO:
    }
    
    fn handle_number(&mut self) {
        // TODO:
    }
    
    fn handle_identifier(&mut self) {
        // TODO:
    }

    // --- scanner functions

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_token_lexeme("", type_);
    }

    fn add_token_lexeme(&mut self, lexeme: &str, type_: TokenType) {
        self.tokens.push(Token::new(type_, &lexeme, self.line));
    }

    fn advance(&mut self) -> char {
        let i = self.current;
        self.current += 1;

        self.source[i]
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            return '\0';
        }

        let i = self.current + 1;
        self.source[i]
    }

    fn match_(&self, ch: char) -> bool {
        ch == self.peek()
    }

    fn match_number(&self) -> bool {
        let ch = self.peek();

        is_digit(ch)
    }
    
    fn finalize(&mut self) {
        self.tokens.push(Token::new(EOF, "", self.line));
    }
    
}


fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
}

fn is_alpha(ch: char) -> bool {
       'a' <= ch && ch <= 'z'
    || 'A' <= ch && ch <= 'Z'
    || ch == '_'
}

fn chars_to_str(chars: &Vec<char>) -> String {
    chars.iter().cloned().collect::<String>()
}
