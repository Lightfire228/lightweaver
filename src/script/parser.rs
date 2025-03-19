use super::tokens::{Token, TokenType::{self, *}};


pub struct Parser {
    tokens: Vec<Token>,
    index:  usize,
}

impl Parser {

    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    pub fn parse(tokens: Vec<Token>) {
        let parser = Parser::new(tokens);


    }


    fn match_one(&mut self, type_: TokenType) -> bool {

        let found = self.check(type_);

        if found {
            self.advance();
        }
        
        found
    }

    fn match_all(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.match_one(t) {
                return true;
            }
        }
        
        false
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_eof() {
            return false;
        }
        self.peek().type_ == type_
    }

    fn is_eof(&self) -> bool {
        if self.index >= self.tokens.len() {
            true
        }
        else {
            self.peek().type_ == EOF
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_eof() {
            self.index += self.index;
        }

        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.index -1]
    }

}