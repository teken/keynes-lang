use std::fmt::Debug;

use crate::lexer::Token;

use super::{expressions::{IdentifierLiteral, Expression}, node::Node};

use dyn_clone::DynClone;


pub trait Statement: Node + DynClone {
    fn statement_node(&self);
}

dyn_clone::clone_trait_object!(Statement);

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub mutable: bool,
    pub name: IdentifierLiteral,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}


#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for ExpressionStatement {}
impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for ReturnStatement {}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}