use std::{thread::sleep, time::Duration};
use log::*;


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL(String),
    EOF,

    IDENT(String),
    INTEGER(String),

    ASSIGN,

    EQUAL,
    NOT_EQUAL,

    GREATER_THAN,
    GREATER_THAN_EQUAL,
    LESS_THAN,
    LESS_THAN_EQUAL,



    RANGE,

    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,

    COMMA,
    SEMICOLEN,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    FUNCTION,
    RETURN,
    MUT,
    LET,

    RUN,
    SPAWN,
}

#[derive(Default)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    peek: char,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok == Token::EOF {
            None
        } else {
            Some(tok)
        }
    }
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input.trim().to_string(),
            ..Default::default()
        };
        trace!("input length {}", lex.input.chars().count());
        trace!("input {:?}", lex.input.chars());
        trace!("new ");
        lex.read_char();
        lex
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.chars().count() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };

        self.peek = if self.read_position + 1 >= self.input.chars().count() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position + 1).unwrap()
        };

        trace!("read_char: {} {}", self.ch, self.read_position);

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_previous_char(&mut self) {
        self.ch = if self.position == 0 {
            '\0'
        } else {
            self.input.chars().nth(self.position - 1).unwrap()
        };

        self.peek = if self.position == 1 {
            '\0'
        } else {
            self.input.chars().nth(self.position - 2).unwrap()
        };

        trace!("read_previous_char: {} {}", self.ch, self.position);

        self.read_position = self.position;
        self.position -= 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match (self.ch, self.peek) {
            ('!', '=') => {
                self.read_char();
                Token::NOT_EQUAL
            },
            ('=', '=') => {
                self.read_char();
                Token::EQUAL
            },
            ('=', _) => Token::ASSIGN,

            ('>', '=') => {
                self.read_char();
                Token::GREATER_THAN_EQUAL
            },
            ('>', _) => Token::GREATER_THAN,

            ('<', '=') => {
                self.read_char();
                Token::LESS_THAN_EQUAL
            },
            ('<',_) => Token::LESS_THAN,

            ('.', '.') => {
                self.read_char();
                Token::RANGE
            },

            ('+', _) => Token::PLUS,
            ('-', _) => Token::MINUS,
            ('*', _) => Token::MULTIPLY,
            ('/', _) => Token::DIVIDE,

            (';', _) => Token::SEMICOLEN,
            (',', _) => Token::COMMA,

            ('(', _) => Token::LPAREN,
            (')', _) => Token::RPAREN,
            ('{', _) => Token::LBRACE,
            ('}', _) => Token::RBRACE,
            ('[', _) => Token::LBRACKET,
            (']', _) => Token::RBRACKET,

            ('\0', _) => Token::EOF,   
            _ => if self.ch.is_alphabetic() {
                lookup_ident(self.read_identifier())
            } else if self.ch.is_numeric() {
                Token::INTEGER(self.read_number())
            } else {
                Token::ILLEGAL(String::from(self.ch))
            },
        };
        trace!("next_token ");
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        trace!("read_identifier: {}", self.ch);
        let start = self.position;
        while self.ch.is_alphanumeric() {
            trace!("read_identifier loop ");
            self.read_char();
        }
        let end = self.position;
        self.read_previous_char();
        self.input[start..end].to_string()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.ch.is_numeric() {
            trace!("read_number loop ");
            self.read_char();
        }
        let end = self.position;
        self.read_previous_char();
        self.input[start..end].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            trace!("skip_whitespace loop ");
            self.read_char();
        }
    }
}

fn lookup_ident(ident: String) -> Token {
    match ident.as_str() {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "mut" => Token::MUT,
        "return" => Token::RETURN,
        "run" => Token::RUN,
        "spawn" => Token::SPAWN,
        _ => Token::IDENT(ident),
    }
}