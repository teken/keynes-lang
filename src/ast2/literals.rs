use super::traits::Expression;

#[derive(Debug, Clone)]
pub struct IdentifierLiteral {
    pub name: String
}

impl Expression for IdentifierLiteral {}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: String,
    pub length: String,
}

impl Expression for IntegerLiteral {}

#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub value: String,
    pub length: String,
}

impl Expression for FloatLiteral {}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub value: bool
}

impl Expression for BooleanLiteral {}
