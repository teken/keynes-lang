use crate::{lexer::{Lexer, Token}, ast::*};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl <'a> Parser<'a> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
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
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("parse_let_statement");
        let token = self.cur_token.clone();
    
        let mutable = !self.expect_peek(Token::MUT);

        if self.expect_peek_ident().is_none() {
            return None;
        }
        let name = self.cur_token.clone();

        if !self.expect_peek(Token::ASSIGN) {
            return None;
        }

        let expression = self.parse_expression();
        if expression.is_none() {
            return None;
        }
        Some(Box::new(LetStatement {
            token,
            name,
            mutable,
            value: expression.unwrap(),
        }))
    }

    fn parse_expression(&mut self) -> Option<Box<dyn Expression>> {
        match self.cur_token.clone() {
            Token::INTEGER(val) => self.parse_integer_literal(val),
            _ => None,
        }
    }


    fn cur_token_is(&self, t: Token) -> bool {
        self.cur_token == t
    }

    fn peek_token_is(&self, t: Token) -> bool {
        self.peek_token == t
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
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
        let parsed_val = val.parse::<i64>();
        if parsed_val.is_err() {
            return None;
        }
        Some(Box::new(IntegerLiteral {
            token: self.cur_token.clone(),
            value: parsed_val.unwrap(),
        }))
    }
}