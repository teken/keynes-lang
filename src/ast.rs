use std::fmt::Debug;

use crate::lexer::Token;

pub trait Node {}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

impl Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expression")
            .field("expression_node", &"expression_node")
            .finish()
    }
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Statement")
            .field("statement_node", &"statement_node")
            .finish()
    }
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl Node for Program {}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}