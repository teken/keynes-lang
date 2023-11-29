use std::mem::zeroed;

use crate::{lexer::{Lexer, Token}, ast::{statements::*, expressions::*, program::Program}};
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
        trace!("parse_program");
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
        trace!("parse_statement: {:?}", self.cur_token);
        let statment = match self.cur_token {
            Token::RETURN => self.parse_return_statement(),
            Token::LET => self.parse_let_statement(),
            Token::RUN => todo!(),
            Token::SPAWN => todo!(),
            _ => self.parse_expression_statement(),
        };

        trace!("parse_statement completed: {:?}", statment);
        statment
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        trace!("parse_return_statement",);
        let token = self.cur_token.clone();
        self.next_token();
        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            trace!("parse_return_statement: parse_expression failed");
            return None;
        }
        trace!("parse_return_statement: expression {:?}", expression);
        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ReturnStatement {
            token,
            expression: expression.unwrap(),
        }))
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        trace!("parse_let_statement",);
        let token = self.cur_token.clone();
    
        let mutable = self.optional_peek(Token::MUT);

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

        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            trace!("parse_let_statement: parse_expression failed");
            return None;
        }

        trace!("parse_let_statement: expression {:?}", expression);

        Some(Box::new(LetStatement {
            token,
            name: name.into(),
            mutable,
            value: expression.unwrap(),
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        trace!("parse_expression_statement",);
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            trace!("parse_expression_statement: parse_expression failed");
            return None;
        }
        trace!("parse_expression_statement: expression {:?}", expression);
        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ExpressionStatement {
            token,
            expression: expression.unwrap(),
        }))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        trace!("parse_expression: {:?} {:?} {:?}", precedence, self.cur_token, self.peek_token);

        let prefix_w = match self.cur_token {
            Token::IDENTIFIER(_) => self.parse_identifier_expression(),
            Token::INTEGER(_) => self.parse_integer_literal(),
            Token::LPAREN => self.parse_grouped_expression(),
            Token::IF => self.parse_if_expression(),
            Token::BANG | Token::MINUS => self.parse_prefix_expression(),
            Token::FUNCTION => self.parse_function_literial(),
            Token::TRUE | Token::FALSE => self.parse_boolean_literal(),
            // Token::LPAREN => self.parse_call_expression(),
            _ => None,
        };
        if prefix_w.is_none() {
            trace!("parse_expression: prefix failed for {:?}", self.cur_token);
            self.errors.push(format!("unhandled prefix parse for {:?}", self.cur_token));
            return None;
        }

        let prefix = prefix_w.unwrap();

        let mut left_exp = Some(prefix.clone());

        while !self.peek_token_is(&Token::SEMICOLON) && precedence < self.peek_token.clone().into() {
            let infix = match self.peek_token.clone() {
                Token::PLUS | 
                Token::MINUS | 
                Token::DIVIDE | 
                Token::MULTIPLY | 
                Token::EQUAL | 
                Token::NOT_EQUAL | 
                Token::LESS_THAN | 
                Token::GREATER_THAN => {
                    self.next_token();
                    self.parse_infix_expression(prefix.clone())
                }
                _ => None,
            };

            if infix.is_none() {
                return left_exp;
            }

            left_exp = Some(infix.unwrap());
        }

        self.next_token();


        self.check_parser_errors();
        left_exp
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
            Token::IDENTIFIER(_) => {
                self.next_token();
                Some(self.cur_token.clone())
            },
            _ => None,
        }
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> { 
        trace!("parse_integer_literal: {:?}", self.cur_token);
        let value = match self.cur_token.clone() {
            Token::INTEGER(val) => Some(val),
            _ => return None,
        };
        if value.is_none() {
            self.errors.push(format!("could not parse {:?} as integer", self.cur_token));
            return None;
        }

        let parsed_val = value.unwrap().parse::<i64>();
        if parsed_val.is_err() {
            self.errors.push(format!("could not parse {:?} as integer", self.cur_token));
            return None;
        }
        Some(Box::new(IntegerLiteral {
            token: self.cur_token.clone(),
            value: parsed_val.unwrap(),
        }))
    }

    fn parse_identifier_expression(&mut self) -> Option<Box<dyn Expression>> {
        trace!("parse_identifier_expression: {:?}", self.cur_token);
        let val = match self.cur_token.clone() {
            Token::IDENTIFIER(val) => Some(val),
            _ => return None,
        };
        if val.is_none() {
            self.errors.push(format!("could not parse {:?} as identifier", self.cur_token));
            return None;
        }

        Some(Box::new(IdentifierLiteral {
            token: self.cur_token.clone(),
        }))
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        trace!("parse_prefix_expression: {:?}", self.cur_token);
        let token = self.cur_token.clone();

        self.next_token();

        let r_exp = self.parse_expression(Precedence::PREFIX);
        if r_exp.is_none() {
            self.errors.push(format!("expected expression after {:?}", token));
            return None;
        }

        Some(Box::new(PrefixExpression {
            token: token.clone(),
            operator: token.into(),
            right: r_exp.unwrap(),
        }))
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        trace!("parse_infix_expression: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        let precedence: Precedence = token.clone().into();
        
        let token_as_infix = token.clone().try_into();
        if token_as_infix.is_err() {
            self.errors.push(format!("expected infix operator, got {:?}", token));
            return None;
        }

        self.next_token();
        trace!("parse_infix_expression: {:?}", self.cur_token);

        let right = self.parse_expression(precedence);
        if right.is_none() {
            self.errors.push(format!("expected expression after {:?}", token));
            return None;
        }
        Some(Box::new(InfixExpression {
            token: token.clone(),
            operator: token_as_infix.unwrap(),
            left,
            right: right.unwrap(),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
        trace!("parse_grouped_expression: {:?}", self.cur_token);
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(Token::RPAREN) {
            return None;
        }
        exp
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        trace!("parse_if_expression: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        if !self.expect_peek(Token::LPAREN) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST);
        if condition.is_none() {
            return None;
        }
        if !self.expect_peek(Token::RPAREN) {
            return None;
        }
        if !self.expect_peek(Token::LBRACE) {
            return None;
        }
        let consequence = self.parse_block_statement();
        if consequence.is_none() {
            return None;
        }
        let alternative = if self.peek_token_is(&Token::ELSE) {
            self.next_token();
            if !self.expect_peek(Token::LBRACE) {
                return None;
            }
            self.parse_block_statement()
        } else {
            None
        };
        Some(Box::new(IfExpression {
            token,
            condition: condition.unwrap(),
            consequence: consequence.unwrap(),
            alternative,
        }))
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        trace!("parse_block_statement: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        let mut statements = Vec::new();
        self.next_token();
        while !self.cur_token_is(&Token::RBRACE) && !self.cur_token_is(&Token::EOF) {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                statements.push(stmt.unwrap());
            }
            self.next_token();
        }
        Some(BlockStatement {
            token,
            statements,
        })
    }

    fn parse_function_literial(&mut self) -> Option<Box<dyn Expression>> {
        trace!("parse_function_literial: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        if !self.expect_peek(Token::LPAREN) {
            return None;
        }
        let parameters = self.parse_function_parameters();
        if parameters.is_none() {
            return None;
        }
        if !self.expect_peek(Token::LBRACE) {
            return None;
        }
        let body = self.parse_block_statement();
        if body.is_none() {
            return None;
        }
        Some(Box::new(FunctionLiteral {
            token,
            parameters: parameters.unwrap(),
            body: body.unwrap(),
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<IdentifierLiteral>> {
        trace!("parse_function_parameters: {:?}", self.cur_token);
        let mut identifiers = Vec::new();
        if self.peek_token_is(&Token::RPAREN) {
            self.next_token();
            return Some(identifiers);
        }
        self.next_token();
        let ident = match self.cur_token.clone() {
            Token::IDENTIFIER(val) => Some(val),
            _ => return None,
        };
        if ident.is_none() {
            self.errors.push(format!("could not parse {:?} as identifier", self.cur_token));
            return None;
        }
        identifiers.push(IdentifierLiteral {
            token: self.cur_token.clone(),
        });
        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();
            let ident = match self.cur_token.clone() {
                Token::IDENTIFIER(val) => Some(val),
                _ => return None,
            };
            if ident.is_none() {
                self.errors.push(format!("could not parse {:?} as identifier", self.cur_token));
                return None;
            }
            identifiers.push(IdentifierLiteral {
                token: self.cur_token.clone(),
            });
        }
        if !self.expect_peek(Token::RPAREN) {
            return None;
        }
        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        trace!("parse_call_expression: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        let arguments = self.parse_call_arguments();
        if arguments.is_none() {
            return None;
        }
        Some(Box::new(CallExpression {
            token,
            function,
            arguments: arguments.unwrap(),
        }))
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Box<dyn Expression>>> {
        trace!("parse_call_arguments: {:?}", self.cur_token);
        let mut arguments = Vec::new();
        if self.peek_token_is(&Token::RPAREN) {
            self.next_token();
            return Some(arguments);
        }
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if exp.is_none() {
            return None;
        }
        arguments.push(exp.unwrap());
        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();
            let exp = self.parse_expression(Precedence::LOWEST);
            if exp.is_none() {
                return None;
            }
            arguments.push(exp.unwrap());
        }
        if !self.expect_peek(Token::RPAREN) {
            return None;
        }
        Some(arguments)
    }

    fn parse_boolean_literal(&mut self) -> Option<Box<dyn Expression>> {
        trace!("parse_boolean_literal: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        let value = match self.cur_token.clone() {
            Token::TRUE => Some(true),
            Token::FALSE => Some(false),
            _ => None,
        };
        if value.is_none() {
            self.errors.push(format!("could not parse {:?} as boolean", self.cur_token));
            return None;
        }
        Some(Box::new(BooleanLiteral {
            token,
            value: value.unwrap(),
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

#[cfg(test)]
#[path = "./parser_tests.rs"]
mod tests;