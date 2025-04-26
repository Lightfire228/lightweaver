use super::{ast::*, tokens::{Token, TokenType::{self, *}}};

use super::ast::Stmt;


use ParseErrorType::*;


pub struct Parser<'a> {
    iter:   TokenIter<'a>,
    cursor: Option<ParserCursor<'a>>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum ParseErrorType {
    InvalidAssignmentTarget(Token),
    UnexpectedToken        (Token, TokenType, String),

    AtBeginning,
    EOF,
}

pub type ParseResult<T> = Result<T, ParseErrorType>;

impl<'a> Parser<'a> {

    pub fn parse_tokens(tokens: &'a [Token]) -> ParseResult<Ast> {

        let mut parser     = Parser::new(tokens);
        let mut statements = Vec::new();

        parser.next();

        while parser.has_next() {
            statements.push(parser.declaration()?);
        }

        Ok(Ast {
            stmts: statements,
        })
    }

    fn new(tokens: &'a [Token]) -> Self {

        Self {
            cursor: None,
            iter: TokenIter { 
                tokens,
                index:   0,
                current: None,
                next:    tokens.first()
            },
        }
    }


    fn declaration(&mut self) -> ParseResult<Stmt> {

        let cursor = self.cursor()?;

        match cursor.current.type_ {
            LetToken => {
                self.next();
                self.var_declaration()
            },
            _   => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        use super::ast::VarDecl;
        let name   = self.consume(Identifier, "Missing identifier after 'let'")?;

        let initializer = match self.cursor()?.current.type_ {
            Equals => {
                self.next();
                Some(self.expression()?)
            },
            _      => None,
        };

        self.consume(SemiColon, "Missing semicolon")?;

        Ok(VarDecl::new(name, initializer))
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;

        self.consume(SemiColon, "Missing semicolon")?;

        Ok(ExpressionStmt::new(expr))
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
                    Ok(Assign::new(var.name.clone(), value))
                },

                _ => Err(InvalidAssignmentTarget(equals.clone()))
            }
        }

        Ok(expr)
    }

    fn instantiation(&mut self) -> ParseResult<Expr> {

        let cursor = self.cursor()?;

        match cursor.current.type_ {
            RectToken => {
                let token = cursor.current.clone();
                self.next();

                self.consume(LeftCurly,  "Missing '{'")?;
                self.consume(RightCurly, "Missing '}'")?;
        
                Ok(Instantiation::new(token))
            }
            _ => self.connection()
        }

        
    }

    fn connection(&mut self) -> ParseResult<Expr> {
        // TODO: these should be expressions, not identifiers
        let left  = self.consume(Identifier,     "Missing operand") ?;
        let op    = self.consume(RightThinArrow, "Missing operator")?;
        let right = self.consume(Identifier,     "Missing operand") ?;

        Ok(Connection::new(Variable::new(left), op, Variable::new(right)))
    }

    // ----

    fn consume(&mut self, token_type: TokenType, msg: &str) -> ParseResult<Token> {

        let cursor = self.cursor()?;
        
        if !cursor.match_token(token_type) {
            let token = cursor.current.clone();
            return Err(UnexpectedToken(token, token_type, msg.to_owned()))
        }

        let token = self.cursor()?.current.clone();
        self.next();

        Ok(token)
    }


    fn cursor(&self) -> ParseResult<&ParserCursor> {
        self.cursor.as_ref().ok_or(ParseErrorType::EOF)
    }

    fn next(&mut self) -> Option<&ParserCursor> {
        self.cursor = self.iter.next();
        self.cursor.as_ref()
    }

    fn has_next(&self) -> bool {
        self.iter.has_next()
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
    _next:     Option<&'a Token>,
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
            _next: next,
        });

        self.current = Some(current);
        self.next    = next;

        result
    }
}

impl<'a> TokenIter<'a> {
    fn has_next(&self) -> bool {
        self.index < self.tokens.len()
    }
}

impl<'a> ParserCursor<'a> {
    fn match_token(&self, token_type: TokenType) -> bool {
        self.current.type_ == token_type
    }
}



#[cfg(test)]
mod test {
    use crate::script::{parser::Parser, scanner::Scanner, test::get_example_001};


    #[test]
    fn test_token_iter() {
        let example = get_example_001();

        let str = example.source;

        let tokens = Scanner::scan_tokens(&str).unwrap();
        let iter   = Parser::new(&tokens).iter.into_iter();

        assert!(iter.zip(example.tokens).all(|x| {
            x.0.current == &x.1
        }));
    }
    
    #[test]
    fn base() {
        let example = get_example_001();

        let str    = example.source;
        let tokens = Scanner::scan_tokens (&str)   .unwrap();
        let ast    = Parser ::parse_tokens(&tokens).unwrap();

        assert_eq!(ast, example.ast);


    }
}