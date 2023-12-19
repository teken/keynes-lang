use super::{traits::{Expression, Statement}, literals::IdentifierLiteral};

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>
}

impl Statement for ExpressionStatement {}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: IdentifierLiteral,
    pub mutable: bool,
    pub value: Box<dyn Expression>
}

impl Statement for LetStatement {}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Box<dyn Expression>
}

impl Statement for ReturnStatement {}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>
}

impl Statement for BlockStatement {}