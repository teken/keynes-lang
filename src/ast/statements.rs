use std::fmt::{Debug, Display};

use crate::lexer::Token;

use super::{expressions::{IdentifierLiteral, Expression}, node::Node};

use dyn_clone::DynClone;


pub trait Statement: Node + DynClone {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn std::any::Any;
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut let_statement = String::new();
        let_statement.push_str(&format!("{} ", self.token));
        if self.mutable {
            let_statement.push_str("mut ");
        }
        let_statement.push_str(&format!("{} = {};", self.name, self.value));
        write!(f, "{}", let_statement)
    }
}


#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for ExpressionStatement {}
impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
    
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for ReturnStatement {}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut return_statement = String::new();
        return_statement.push_str(&format!("{} ", self.token));
        return_statement.push_str(&format!("{}", self.expression));
        return_statement.push_str(";");
        write!(f, "{}", return_statement)
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {}
impl Statement for BlockStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut block_statement = String::new();
        for statement in &self.statements {
            block_statement.push_str(&format!("{}", statement));
        }
        write!(f, "{{ {} }}", block_statement)
    }
    
}