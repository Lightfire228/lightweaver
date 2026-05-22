use std::{collections::HashMap, num::Wrapping, ops::Index, usize};

use crate::script::tokens::{Token, TokenType};

type Tt = TokenType;

#[derive(Debug)]
pub enum ScannerError {
    UnterminatedString{line: usize, col: usize},
    UnexpectedChar    {line: usize, col: usize, ch: char}
}

type Se = ScannerError;


struct Scanner {
    chars:        Vec<char>,

    cursor:       Cursor, // the previously consumed character
    index:        usize,  // the next character to consume

    lexeme_start: usize,
}

#[derive(Debug, Clone, Copy)]
struct Cursor {
    prv:    char,
    char:   char,
    next:   char,
    next_2: char,

    line:   usize,
    col:    usize,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, ScannerError> {
    let mut scanner = Scanner::new(source);
    let mut tokens  = Vec::new();

    while scanner.has_next() {
        scanner.scan_token(&mut tokens)?;
    }

    tokens.push(Token {
        type_:  Tt::EOF,
        lexeme: String::new(),
        line:   scanner.cursor.line,
        col:    scanner.cursor.col,
    });

    Ok(tokens)
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let chars: Vec<_> = source.chars().collect();

        let cursor = Cursor {
            prv:    '\0',
            char:   '\0',
            next:   chars.get(0).map_or('\0', |x| *x),
            next_2: chars.get(1).map_or('\0', |x| *x),

            line:   1,
            col:    1,

        };

        Scanner {
            chars,
            cursor,
            lexeme_start: 0,
            index:        0,
        }
    }

    pub fn scan_token(&mut self, tokens: &mut Vec<Token>) -> Result<(), ScannerError> {
        self.skip_whitespace();

        self.lexeme_start = self.index;

        if self.next().is_none() {
            return Ok(());
        }

        let ch = self.cursor.char;

        if is_alpha(ch) {
            self.parse_identifier(tokens);
            return Ok(());
        }
        if is_digit(ch) {
            self.parse_number(tokens);
            return Ok(());
        }

        match ch {

            '(' => self.add_token(tokens, Tt::LeftParen),
            ')' => self.add_token(tokens, Tt::RightParen),
            '{' => self.add_token(tokens, Tt::LeftBrace),
            '}' => self.add_token(tokens, Tt::RightBrace),
            ';' => self.add_token(tokens, Tt::Semicolon),
            ',' => self.add_token(tokens, Tt::Comma),
            '.' => self.add_token(tokens, Tt::Dot),
            '-' => self.add_token(tokens, Tt::Minus),
            '+' => self.add_token(tokens, Tt::Plus),
            '/' => self.add_token(tokens, Tt::Slash),
            '*' => self.add_token(tokens, Tt::Star),

            '!' => self.add_token_match(tokens, '=', Tt::Bang,    Tt::BangEqual),
            '=' => self.add_token_match(tokens, '=', Tt::Equal,   Tt::EqualEqual),
            '<' => self.add_token_match(tokens, '=', Tt::Less,    Tt::LessEqual),
            '>' => self.add_token_match(tokens, '=', Tt::Greater, Tt::GreaterEqual),

            '"' => self.parse_string(tokens)?,

            _   => Err(Se::UnexpectedChar {
                line: self.cursor.line,
                col:  self.cursor.col,
                ch
            })?,
        }

        Ok(())
    }

    fn parse_identifier(&mut self, tokens: &mut Vec<Token>) {
        while is_alpha_numeric(self.cursor.next) {
            self.next();
        }

        let lexeme = self.get_lexeme();

        let type_  = *get_keywords().get(&lexeme).unwrap_or(&Tt::Identifier);

        tokens.push(self.new_token(type_, lexeme));
    }

    fn parse_number(&mut self, tokens: &mut Vec<Token>) {
        while is_digit(self.cursor.next) {
            self.next();
        }

        if self.cursor.next == '.' && is_digit(self.cursor.next_2) {
            self.next();

            while is_digit(self.cursor.next) {
                self.next();
            }
        }

        self.add_token(tokens, Tt::Number);
    }

    fn parse_string(&mut self, tokens: &mut Vec<Token>) -> Result<(), ScannerError> {

        loop {
            if self.cursor.char == '\\' {
                self.next();
            }

            if self.next().is_none() {
                return Err(ScannerError::UnterminatedString { line: self.cursor.line, col: self.cursor.col });
            }

            if self.cursor.char == '"' {
                break;
            }
        }

        self.add_str_token(tokens);

        Ok(())

    }

    fn add_token(&mut self, tokens: &mut Vec<Token>, type_: TokenType) {
        tokens.push(
            self.new_token(type_, self.get_lexeme())
        );
    }

    fn add_str_token(&mut self, tokens: &mut Vec<Token>) {
        tokens.push(
            self.new_token(Tt::String, self.get_wrapped_lexeme())
        );
    }

    fn add_token_match(&mut self, tokens: &mut Vec<Token>, ch: char, first: TokenType, second: TokenType) {
        let type_ = if self.cursor.next == ch {
            self.next();
            second
        } else {
            first
        };

        tokens.push(self.new_token(type_, self.get_lexeme()));
    }

    fn new_token(&mut self, type_: TokenType, lexeme: String) -> Token {
        self.lexeme_start = self.index -1;

        Token {
            line: self.cursor.line,
            col:  self.cursor.col - lexeme.len(),
            type_,
            lexeme,
        }
    }

    fn get_lexeme(&self) -> String {
        chars_to_str(&self.chars[self.lexeme_start..self.index])
    }
    fn get_wrapped_lexeme(&self) -> String {
        chars_to_str(&self.chars[self.lexeme_start+1..self.index-1])
    }

    fn skip_whitespace(&mut self) {

        loop {

            if self.cursor.next == '/' && self.cursor.next_2 == '/' {
                self.skip_to_eol();
            }

            match self.cursor.next {
                  ' '
                | '\r'
                | '\n'
                | '\t' => {},

                _ => return,
            }

            self.next();
        }
    }

    fn skip_to_eol(&mut self) {
        while self.next().is_some_and(|cur| cur.char != '\n') {}
    }

    fn has_next(&self) -> bool {
        self.index < self.chars.len()
    }
}


impl Iterator for Scanner {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {

        if !self.has_next() {
            return None;
        }

        let next = self.chars.get(self.index +2).map_or(0 as char, |x| *x);


        self.cursor = Cursor {
            prv:    self.cursor.char,
            char:   self.cursor.next,
            next:   self.cursor.next_2,
            next_2: next,

            line:   self.cursor.line,
            col:    self.cursor.col   +1,
        };

        if self.cursor.prv == '\n' {
            self.cursor.line += 1;
            self.cursor.col   = 1;
        }

        self.index += 1;

        Some(self.cursor)
    }
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

    use super::*;



    #[test]
    fn base() {
        let example = get_example_001();

        let tokens = tokenize(&example.source);
        dbg!("{}", &tokens);

        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        for (t, e) in tokens.iter().zip(example.tokens) {
            dbg!("{}, {}", &t, &e);
            assert_eq!(t.type_,  e.type_);
            assert_eq!(t.lexeme, e.lexeme);
            assert_eq!(t.line,   e.line);
            // assert_eq!(t.col,    e.col);
        }

    }

}
