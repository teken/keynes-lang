use std::fmt::Display;

use super::{node::Node, statements::Statement};

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

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut program = String::new();
        for statement in &self.statements {
            program.push_str(&format!("{}", statement));
        }
        write!(f, "{}", program)
    }
}