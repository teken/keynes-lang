use std::fmt::{Debug, Display};

use crate::lexer::Token;

use super::{node::Node, statements::BlockStatement};

use dyn_clone::DynClone;

pub trait Expression: Node + DynClone {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn std::any::Any;
}

dyn_clone::clone_trait_object!(Expression);

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
    
}


#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierLiteral {
    pub token: Token,
}

impl Node for IdentifierLiteral {}
impl Expression for IdentifierLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for IdentifierLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
    
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOperator {
    BANG,
    MINUS,
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOperator::BANG => write!(f, "!"),
            PrefixOperator::MINUS => write!(f, "-"),
        }
    }
    
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

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    LOWEST,
    EQUALS      ,// ==
    LESSGREATER ,// > or <
    SUM         ,// +
    PRODUCT     ,// *
    PREFIX      ,// -X or !X
    CALL        ,// myFunction(X)
}

impl Precedence {
    pub fn reduce(self) -> Precedence {
        match self {
            Precedence::EQUALS => Precedence::LOWEST,
            Precedence::LESSGREATER => Precedence::EQUALS,
            Precedence::SUM => Precedence::LESSGREATER,
            Precedence::PRODUCT => Precedence::SUM,
            Precedence::PREFIX => Precedence::PRODUCT,
            Precedence::CALL => Precedence::PREFIX,
            Precedence::LOWEST => Precedence::LOWEST,
        }
    }
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
    
}

#[derive(Debug, Clone, PartialEq)]
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

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::PLUS => write!(f, "+"),
            InfixOperator::MINUS => write!(f, "-"),
            InfixOperator::MULTIPLY => write!(f, "*"),
            InfixOperator::DIVIDE => write!(f, "/"),
            InfixOperator::EQUAL => write!(f, "=="),
            InfixOperator::NOT_EQUAL => write!(f, "!="),
            InfixOperator::LESS_THAN => write!(f, "<"),
            InfixOperator::LESS_THAN_EQUAL => write!(f, "<="),
            InfixOperator::GREATER_THAN => write!(f, ">"),
            InfixOperator::GREATER_THAN_EQUAL => write!(f, ">="),
        }
    }
    
}

impl TryFrom<Token> for InfixOperator {
    type Error = String;
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::PLUS => Ok(InfixOperator::PLUS),
            Token::MINUS => Ok(InfixOperator::MINUS),
            Token::MULTIPLY => Ok(InfixOperator::MULTIPLY),
            Token::DIVIDE => Ok(InfixOperator::DIVIDE),
            Token::EQUAL => Ok(InfixOperator::EQUAL),
            Token::NOT_EQUAL => Ok(InfixOperator::NOT_EQUAL),
            Token::LESS_THAN => Ok(InfixOperator::LESS_THAN),
            Token::LESS_THAN_EQUAL => Ok(InfixOperator::LESS_THAN_EQUAL),
            Token::GREATER_THAN => Ok(InfixOperator::GREATER_THAN),
            Token::GREATER_THAN_EQUAL => Ok(InfixOperator::GREATER_THAN_EQUAL),
            _ => Err(format!("Invalid infix operator token {:?}", token)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl Node for BooleanLiteral {}
impl Expression for BooleanLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value { "true" } else { "false" })
    }
    
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut if_expression = String::new();
        if_expression.push_str(&format!("{} ", self.token));
        if_expression.push_str(&format!("({}) ", self.condition));
        if_expression.push_str(&format!("{{{}}} ", self.consequence));
        if let Some(alternative) = &self.alternative {
            if_expression.push_str(&format!("else {{{}}}", alternative));
        }
        write!(f, "{}", if_expression)
    }
    
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut function_literal = String::new();
        function_literal.push_str(&format!("{}(", self.token));
        for (i, parameter) in self.parameters.iter().enumerate() {
            function_literal.push_str(&format!("{}", parameter));
            if i < self.parameters.len() - 1 {
                function_literal.push_str(", ");
            }
        }
        function_literal.push_str(&format!(") {}", self.body));
        write!(f, "{}", function_literal)
    }
    
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut call_expression = String::new();
        call_expression.push_str(&format!("{}(", self.function));
        for (i, argument) in self.arguments.iter().enumerate() {
            call_expression.push_str(&format!("{}", argument));
            if i < self.arguments.len() - 1 {
                call_expression.push_str(", ");
            }
        }
        call_expression.push_str(")");
        write!(f, "{}", call_expression)
    }
    
}