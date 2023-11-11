use std::fmt::Debug;

use crate::lexer::Token;

pub trait Node: std::fmt::Debug {}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub mutable: bool,
    pub name: Token,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}