use crate::script::ast::{Binary, Class};

use super::{ast::{Ast, Expr, FunctionStmt, Stmt, Variable}, tokens::{Token, TokenType::{self, *}}};

type ParseResult<T> = Result<T, ParseError>;

pub fn parse_ast(tokens: Vec<Token>) -> ParseResult<Ast> {

    let mut parser = Parser::new(tokens);

    let mut statements = vec![];
    let mut errors     = vec![];
    
    while !parser.is_eof() {
        match parser.parse_declaration() {
            Ok (stmt) => statements.push(stmt),
            Err(err)  => errors    .push(err),
        }
    }

    if errors.len() > 0 {
        Err(errors)
    }
    else {
        Ok(Ast { stmts: statements })
    }
}

struct Parser {
    tokens:  Vec<Token>,
    current: usize,
}


pub struct ParseError {
    type_: ParseErrorType,
    token: Token,

}

pub enum ParseErrorType {
    MissingClassName,
    MissingSuperclassName,
    MissingClassOpenCurly,
    MissingClassCloseCurly,
}

use ParseErrorType::*;

pub enum FunctionType {
    Function,
    Method,
}

impl Parser {

    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn parse_declaration(&mut self) -> ParseResult<Stmt> {
        let result: ParseResult<Stmt> = match self.peek().type_ {

            TokenClass => { self.advance(); self.parse_class_declaration() },
            TokenFun   => { self.advance(); self.parse_function(FunctionType::Function).map(|x| Ok(Stmt::Function(x)))?},
            TokenVar   => { self.advance(); self.parse_var_declaration() },

            _ => self.parse_statement(),
        };

        if result.is_err() {
            self.synchronize();
        }

        result

    }

    fn parse_class_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenIdentifier, MissingClassName)?.to_owned();

        let mut superclass = None;
        if self.match_(vec![TokenLess]) {

            self.consume(TokenIdentifier, MissingSuperclassName)?;
            let name = self.previous().to_owned();
            superclass = Some(Variable { name, });
        }

        self.consume(TokenLeftBrace, MissingClassOpenCurly)?;

        let mut methods = vec![];
        while !self.check(TokenRightBrace) && !self.is_eof() {
            methods.push(self.parse_function(FunctionType::Method)?);
        }

        self.consume(TokenRightBrace, MissingClassCloseCurly)?;

        Ok(Class::new(name, superclass, methods))
    }

    fn parse_function(&mut self, type_: FunctionType) -> ParseResult<FunctionStmt> {
        todo!()
    }

    fn parse_var_declaration(&mut self) -> ParseResult<Stmt> {
        todo!()
    }



    fn parse_statement(&mut self) -> ParseResult<Stmt> {

        match self.peek().type_ {
            TokenFor       => { self.advance(); self.parse_for_statement() },
            TokenIf        => { self.advance(); self.parse_if_statement() },
            TokenPrint     => { self.advance(); self.parse_print_statement() },
            TokenReturn    => { self.advance(); self.parse_return_statement() },
            TokenWhile     => { self.advance(); self.parse_while_statement() },
            TokenLeftBrace => { self.advance(); self.parse_block_statement() },

            _ => self.parse_expression_statement(),
        }
    }

    fn parse_for_statement(&mut self) -> ParseResult<Stmt> {
        todo!()
    }
    fn parse_if_statement(&mut self) -> ParseResult<Stmt> {
        todo!()
    }
    fn parse_print_statement(&mut self) -> ParseResult<Stmt> {
        todo!()
    }
    fn parse_return_statement(&mut self) -> ParseResult<Stmt> {
        todo!()
    }
    fn parse_while_statement(&mut self) -> ParseResult<Stmt> {
        todo!()
    }
    fn parse_block_statement(&mut self) -> ParseResult<Stmt> {
        todo!()
    }
    fn parse_expression_statement(&mut self) -> ParseResult<Stmt> {
        todo!()
    }


    fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    fn parse_equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_comparison()?;

        while self.match_(vec![TokenBang, TokenBangEqual]) {
            let operator = self.previous().to_owned();
            let right    = self.parse_comparison()?;

            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_term()?;

        while self.match_(vec![TokenGreater, TokenGreaterEqual, TokenLess, TokenLessEqual]) {
            let operator = self.previous().to_owned();
            let right    = self.parse_term()?;

            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    fn parse_factor(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    // Utility functions

    // todo: try to rustify this

    fn match_(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_eof() {
            self.current += 1;
        }

        self.previous()
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_eof() {
            return false;
        }
        self.peek().type_ == type_
    }

    fn is_eof(&self) -> bool {
        self.peek().type_ == TokenEOF || self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current -1]
    }

    fn consume(&mut self, token_type: TokenType, error_type: ParseErrorType) -> ParseResult<&Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        self.error(error_type)
    }

    fn error<T>(&mut self, type_: ParseErrorType) -> ParseResult<T> {
        Err(ParseError { 
            type_,
            token: self.previous().to_owned(),
        })

    }

    fn synchronize(&mut self) {

        while !self.is_eof() {
            if self.previous().type_ == TokenSemicolon {
                return;
            }

            match self.peek().type_ {
                  TokenClass
                | TokenFun
                | TokenVar
                | TokenFor
                | TokenIf
                | TokenWhile
                | TokenPrint
                | TokenReturn => {
                    return;
                }

                _ => {}
            }

            self.advance();
        }

    }
}
