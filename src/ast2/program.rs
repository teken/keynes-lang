use super::traits::Statement;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}