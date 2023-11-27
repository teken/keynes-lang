use std::fmt::Debug;

use crate::lexer::Token;

use super::{node::Node, statements::BlockStatement};

use dyn_clone::DynClone;

pub trait Expression: Node + DynClone {
    fn expression_node(&self);
}

dyn_clone::clone_trait_object!(Expression);

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}


#[derive(Debug, Clone)]
pub struct IdentifierLiteral {
    pub token: Token,
}

impl Node for IdentifierLiteral {}
impl Expression for IdentifierLiteral {
    fn expression_node(&self) {}
}

impl From<Token> for IdentifierLiteral {
    fn from(token: Token) -> Self {
        match token.clone() {
            Token::IDENTIFIER(_) => IdentifierLiteral { token },
            _ => panic!("Invalid identifier token {:?}", token),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: PrefixOperator,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {}
impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub enum PrefixOperator {
    BANG,
    MINUS,
}

impl From<Token> for PrefixOperator {
    fn from(token: Token) -> Self {
        match token {
            Token::BANG => PrefixOperator::BANG,
            Token::MINUS => PrefixOperator::MINUS,
            _ => panic!("Invalid prefix operator token {:?}", token),
        }
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS      ,// ==
    LESSGREATER ,// > or <
    SUM         ,// +
    PRODUCT     ,// *
    PREFIX      ,// -X or !X
    CALL        ,// myFunction(X)
}

impl From<Token> for Precedence {
    fn from(token: Token) -> Self {
        match token {
            Token::EQUAL => Precedence::EQUALS,
            Token::NOT_EQUAL => Precedence::EQUALS,
            Token::GREATER_THAN => Precedence::LESSGREATER,
            Token::GREATER_THAN_EQUAL => Precedence::LESSGREATER,
            Token::LESS_THAN => Precedence::LESSGREATER,
            Token::LESS_THAN_EQUAL => Precedence::LESSGREATER,
            Token::PLUS => Precedence::SUM,
            Token::MINUS => Precedence::SUM,
            Token::MULTIPLY => Precedence::PRODUCT,
            Token::DIVIDE => Precedence::PRODUCT,
            Token::LPAREN => Precedence::CALL,
            _ => Precedence::LOWEST,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub operator: InfixOperator,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {}
impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub enum InfixOperator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EQUAL,
    NOT_EQUAL,
    LESS_THAN,
    LESS_THAN_EQUAL,
    GREATER_THAN,
    GREATER_THAN_EQUAL,
}

impl From<Token> for InfixOperator {
    fn from(token: Token) -> Self {
        match token {
            Token::PLUS => InfixOperator::PLUS,
            Token::MINUS => InfixOperator::MINUS,
            Token::MULTIPLY => InfixOperator::MULTIPLY,
            Token::DIVIDE => InfixOperator::DIVIDE,
            Token::EQUAL => InfixOperator::EQUAL,
            Token::NOT_EQUAL => InfixOperator::NOT_EQUAL,
            Token::LESS_THAN => InfixOperator::LESS_THAN,
            Token::LESS_THAN_EQUAL => InfixOperator::LESS_THAN_EQUAL,
            Token::GREATER_THAN => InfixOperator::GREATER_THAN,
            Token::GREATER_THAN_EQUAL => InfixOperator::GREATER_THAN_EQUAL,
            _ => panic!("Invalid infix operator token {:?}", token),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl Node for BooleanLiteral {}
impl Expression for BooleanLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {}
impl Expression for IfExpression {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<IdentifierLiteral>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {}
impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {}
impl Expression for CallExpression {
    fn expression_node(&self) {}
}