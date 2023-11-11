use crate::{lexer::{Lexer, Token}, ast::*};
use log::*;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl <'a> Parser<'a> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self.cur_token != Token::EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
       let statment = match self.cur_token {
            Token::LET => self.parse_let_statement(),
            _ => None,
        };

        trace!("parse_statement: {:?}", statment);
        statment
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        trace!("parse_let_statement",);
        let token = self.cur_token.clone();
    
        let mutable = !self.optional_peek(Token::MUT);

        trace!("parse_let_statement: mutable {:?}", mutable);

        if self.expect_peek_ident().is_none() {
            trace!("parse_let_statement: expect_peek_ident for name failed");
            return None;
        }
        let name = self.cur_token.clone();

        trace!("parse_let_statement: name {:?}", name);

        if !self.expect_peek(Token::ASSIGN) {
            trace!("parse_let_statement: expect_peek for assign failed");
            return None;
        }

        trace!("parse_let_statement: assign");
        self.next_token();

        let expression = self.parse_expression();
        if expression.is_none() {
            trace!("parse_let_statement: parse_expression failed");
            return None;
        }

        trace!("parse_let_statement: expression {:?}", expression);

        Some(Box::new(LetStatement {
            token,
            name,
            mutable,
            value: expression.unwrap(),
        }))
    }

    fn parse_expression(&mut self) -> Option<Box<dyn Expression>> {
        trace!("parse_expression: {:?}", self.cur_token);
        let expression = match self.cur_token.clone() {
            Token::INTEGER(val) => self.parse_integer_literal(val),
            _ => None,
        };
        self.check_parser_errors();
        trace!("parse_expression: {:?}", expression);
        expression
    }


    fn cur_token_is(&self, t: &Token) -> bool {
        self.cur_token == *t
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        self.peek_token == *t
    }

    fn peek_error(&mut self, t: Token) {
        self.errors.push(format!("expected next token to be {:?}, got {:?} instead", t, self.peek_token));
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn optional_peek(&mut self, t: Token) -> bool {
        let r = self.peek_token_is(&t);
        if r {
            self.next_token();
        }
        r
    }

    fn expect_peek_ident(&mut self) -> Option<Token> {
        match self.peek_token {
            Token::IDENT(_) => {
                self.next_token();
                Some(self.cur_token.clone())
            },
            _ => None,
        }
    }

    fn parse_integer_literal(&mut self, val: String) -> Option<Box<dyn Expression>> {
        trace!("parse_integer_literal: {:?}", val);
        let parsed_val = val.parse::<i64>();
        if parsed_val.is_err() {
            return None;
        }
        Some(Box::new(IntegerLiteral {
            token: self.cur_token.clone(),
            value: parsed_val.unwrap(),
        }))
    }

    fn check_parser_errors(&mut self) {
        if self.errors.len() == 0 {
            return;
        }
        error!("parser has {} errors", self.errors.len());
        for error in &self.errors {
            error!("{}", error);
        }
        panic!("parser has {} errors", self.errors.len());
    }
}