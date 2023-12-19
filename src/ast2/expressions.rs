use super::statements::BlockStatement;

use super::traits::Expression;


#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub operator: String,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub conditions: Vec<(Box<dyn Expression>, BlockStatement)>,
    pub alternative: Option<BlockStatement>,
}

impl Expression for IfExpression {}