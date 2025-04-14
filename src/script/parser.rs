use super::{ast::*, tokens::{Token, TokenType::{self, *}}};

use super::ast::Stmt;



use ParseErrorType::*;


pub struct Parser<'a> {
    iter:   TokenIter<'a>,
    cursor: Option<ParserCursor<'a>>,
}

#[derive(Debug)]
pub enum ParseErrorType {
    InvalidAssignmentTarget(Token),
    UnexpectedToken        (Token, TokenType),

    AtBeginning,
    EOF,
}

pub type ParseResult<T> = Result<T, ParseErrorType>;

impl<'a> Parser<'a> {

    pub fn parse_tokens(tokens: &'a [Token]) -> ParseResult<Ast> {

        let mut parser     = Parser::new(tokens);
        let mut statements = Vec::new();

        while parser.has_next() {
            statements.push(parser.declaration()?);
        }

        Ok(Ast {
            stmts: statements,
        })
    }

    fn new(tokens: &'a [Token]) -> Self {

        let mut iter = TokenIter { 
            tokens,
            index:   0,
            current: None,
            next:    tokens.first()
        };

        Self {
            cursor: iter.next(),
            iter,
        }
    }

    fn next(&mut self) -> Option<&ParserCursor> {
        self.cursor = self.iter.next();
        if self.cursor.is_some() {
            dbg!(self.cursor.as_ref().unwrap().current);
        }
        self.cursor.as_ref()
    }

    fn has_next(&self) -> bool {
        self.iter.has_next()
    }


    fn declaration(&mut self) -> ParseResult<Stmt> {

        let cursor = self.cursor()?;

        match cursor.current.type_ {
            Let => {
                self.next();
                self.varDeclaration()
            },
            _   => self.statement(),
        }
    }

    fn varDeclaration(&mut self) -> ParseResult<Stmt> {
        use super::ast::Let;
        let name   = self.consume(Identifier, "")?;

        let initializer = match self.cursor()?.current.type_ {
            Equals => {
                self.next();
                Some(self.expression()?)
            },
            _      => None,
        };

        self.consume(SemiColon, "")?;

        Ok(Let::new(&name, initializer))
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;

        self.consume(SemiColon, "")?;

        Ok(Expression::new(expr))
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr   = self.instantiation()?;

        let cursor = self.cursor()?;

        if cursor.match_token(Equals) {

            let equals = cursor.previous.ok_or(AtBeginning)?.clone();
            let value  = self.assignment()?;

            return match expr {
                Expr::Variable(ref var) => {
                    self.next();
                    Ok(Assign::new(&var.name, value))
                },

                _ => Err(InvalidAssignmentTarget(equals.clone()))
            }
        }

        Ok(expr)
    }

    fn instantiation(&mut self) -> ParseResult<Expr> {

        let cursor = self.cursor()?;

        match cursor.current.type_ {
            Rect => {
                let token = cursor.current.clone();
                
                self.next();

                self.consume(LeftCurly,  "")?;
                self.consume(RightCurly, "")?;
        
                Ok(Instantiation::new(&token))
            }
            _ => self.connection()
        }

        
    }

    fn connection(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    // ----

    fn consume(&mut self, token_type: TokenType, msg: &str) -> ParseResult<Token> {

        let cursor = self.cursor()?;
        
        if !cursor.match_token(token_type) {
            let token = cursor.current.clone();
            return Err(UnexpectedToken(token, token_type))
        }

        self.next();

        let token = self.cursor()?.current.clone();
        Ok(token)
    }

    fn match_token(&mut self, token_type: TokenType) -> ParseResult<bool> {
        Ok(self.cursor()?.current.type_ == token_type)
    }

    fn cursor(&self) -> ParseResult<&ParserCursor> {
        self.cursor.as_ref().ok_or(ParseErrorType::EOF)
    }
}


struct TokenIter<'a> {
    tokens: &'a[Token],
    index:  usize,

    current: Option<&'a Token>,
    next:    Option<&'a Token>,

}

struct ParserCursor<'a> {
    current:  &'a Token,
    previous: Option<&'a Token>,
    next:     Option<&'a Token>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = ParserCursor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;

        let next = self.tokens.get(self.index +1);

        self.index += 1;

        let result = Some(ParserCursor {
            previous: self.current,
            current,
            next,
        });

        self.current = Some(current);
        self.next    = next;

        result
    }
}

impl<'a> TokenIter<'a> {
    fn has_next(&self) -> bool {
        self.index <= self.tokens.len()
    }
}

impl<'a> ParserCursor<'a> {
    fn match_token(&self, token_type: TokenType) -> bool {
        self.current.type_ == token_type
    }
}



#[cfg(test)]
mod test {
    use crate::{multi_line, script::{parser::Parser, scanner::Scanner, tokens::{self, Token}}};

    #[test]
    fn test_token_iter() {
        use tokens::TokenType::*;

        let str = multi_line!(
            "let a = Rect {};",
            "let b = Rect {};",
            "a -> b;",
        );

        let tokens = Scanner::scan_tokens(&str).unwrap();

        let mut iter = Parser::new(&tokens).iter;

        let mut get = || iter.next().map(|x| x.current);

        assert_eq!(get(), Some(&Token::new(Let,             "let",  1)));
        assert_eq!(get(), Some(&Token::new(Identifier,      "a",    1)));
        assert_eq!(get(), Some(&Token::new(Equals,          "=",    1)));
        assert_eq!(get(), Some(&Token::new(Rect,            "Rect", 1)));
        assert_eq!(get(), Some(&Token::new(LeftCurly,       "{",    1)));
        assert_eq!(get(), Some(&Token::new(RightCurly,      "}",    1)));
        assert_eq!(get(), Some(&Token::new(SemiColon,       ";",    1)));
        assert_eq!(get(), Some(&Token::new(Let,             "let",  2)));
        assert_eq!(get(), Some(&Token::new(Identifier,      "b",    2)));
        assert_eq!(get(), Some(&Token::new(Equals,          "=",    2)));
        assert_eq!(get(), Some(&Token::new(Rect,            "Rect", 2)));
        assert_eq!(get(), Some(&Token::new(LeftCurly,       "{",    2)));
        assert_eq!(get(), Some(&Token::new(RightCurly,      "}",    2)));
        assert_eq!(get(), Some(&Token::new(SemiColon,       ";",    2)));
        assert_eq!(get(), Some(&Token::new(Identifier,      "a",    3)));
        assert_eq!(get(), Some(&Token::new(RightThinArrow,  "->",   3)));
        assert_eq!(get(), Some(&Token::new(Identifier,      "b",    3)));
        assert_eq!(get(), Some(&Token::new(SemiColon,       ";",    3)));
        assert_eq!(get(), Some(&Token::new(EOF,             "",     3)));
        assert_eq!(get(), None);

    }
    
    #[test]
    fn base() {
        let str = multi_line!(
            "let a = Rect {};",
            "let b = Rect {};",
            "a -> b;",
        );

        let tokens = Scanner::scan_tokens(&str).unwrap();

        let ast = Parser::parse_tokens(&tokens).unwrap();

        dbg!(ast.stmts);
        panic!()

        // let first_line = Stmt::Let(
        //     lt {
        //         name:        Token::new(Identifier, "a", 1),
        //         initializer: Box::new(Expr::Assign(Assign {
        //             type_: Token::new(Identifier, "Rect", 1),
        //             // body:  Box::new(Body {
        //             //     properties: vec![],
        //             // }),
        //         }))
        //     }
        // );

        // let second_line = Stmt::Let(
        //     lt {
        //         name:        Token::new(Identifier, "b", 2),
        //         initializer: Box::new(Expr::Instantiation(Instantiation {
        //             type_: Token::new(Identifier, "Rect", 2),
        //             // body:  Box::new(Body {
        //             //     properties: vec![],
        //             // }),
        //         }))
        //     }
        // );

        // // let third_line = Stmt::Expression(Expr::)

        // assert_eq!(ast, Ast {
        //     stmts: vec![
                
        //     ]
        // });

    }
}