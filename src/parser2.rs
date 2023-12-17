use std::fmt::Debug;

use nom::{
    branch::alt,
    IResult,
    combinator::{
        opt, map, fail, map_res,
    },
    bytes::complete::{
        tag_no_case,
        tag,
        take_while1, take_while, take,
    },
    character::complete::{
        multispace0,
        multispace1,
    }, multi::many1, sequence::delimited
};
use nom_7_precedence::{precedence, binary_op, Assoc, unary_op, Operation};

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, statements) = many1(parse_statment)(input)?;
    Ok((input, Program {
        statements
    }))
}

fn parse_statment(input: &str) -> IResult<&str, Box<dyn Statement>> {
    Ok(alt((
        parse_let_statement,
        parse_return_statement,
        // parse_expression_statement
    ))(input)?)
}

fn parse_let_statement(input: &str) -> IResult<&str, Box<dyn Statement>> {
    let (input, _) = tag("let")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, mutable) = opt(tag_no_case("mut"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_identifier_literal(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, _) = tag(";")(input)?;
    Ok((input, Box::new(LetStatement 
        { mutable: mutable.is_some(), name, value })))
}

fn parse_return_statement(input: &str) -> IResult<&str, Box<dyn Statement>> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value) = parse_expression(input)?;
    Ok((input, Box::new(ReturnStatement { value })))
}

fn parse_identifier_literal(input: &str) -> IResult<&str, IdentifierLiteral> {
    let (input, name) = take_while1(|c: char| c.is_alphanumeric())(input)?;
    Ok((input, IdentifierLiteral { name: name.to_string() }))
}

fn parse_integer_literal(input: &str) -> IResult<&str, IntegerLiteral> {
    let (input, value) = take_while1(|c: char| c.is_numeric() || c == '_')(input)?;
    let (input, _) = tag("i")(input)?;
    let (input, length) = take_while(|c: char| c.is_numeric() || c == '_')(input)?;
    Ok((input, IntegerLiteral {
        value: value.to_string(),
        length: length.to_string(),
    }))
}

fn parse_boolean_literal(input: &str) -> IResult<&str, BooleanLiteral> {
    let (input, exp) = alt((
        map(tag("true"), |_| BooleanLiteral { value: true }),
        map(tag("false"), |_| BooleanLiteral { value: false })
    ))(input)?;
    Ok((input, exp))
}

fn parse_expression(input: &str) -> IResult<&str, Box<dyn Expression>> {
    precedence(
alt((
            unary_op(1, tag("-")),
            unary_op(1, tag("!")),
        )),
        fail,
        alt((
            binary_op(2, Assoc::Left, tag("*")),
            binary_op(2, Assoc::Left, tag("/")),
            binary_op(3, Assoc::Left, tag("+")),
            binary_op(3, Assoc::Left, tag("-")),
            binary_op(4, Assoc::Left, tag("==")),
            binary_op(4, Assoc::Left, tag("!=")),
            binary_op(4, Assoc::Left, tag(">")),
            binary_op(4, Assoc::Left, tag(">=")),
            binary_op(4, Assoc::Left, tag("<")),
            binary_op(4, Assoc::Left, tag("<=")),
        )),
        alt((
            delimited(tag("("), parse_expression, tag(")")),
            map(parse_integer_literal, |i| Box::new(i) as Box<dyn Expression>),
            map(parse_boolean_literal, |b| Box::new(b) as Box<dyn Expression>),
            map(parse_identifier_literal, |i| Box::new(i) as Box<dyn Expression>),
       )),
        |op: Operation<&str, &str, &str, Box<dyn Expression>>| { //evaluating the expression step by step
            match op {
                Operation::Prefix(op, o) => Ok(Box::new(PrefixExpression {
                    operator: op.to_string(),
                    right: o,
                })),
                Operation::Binary(lhs, op, rhs) => Ok(Box::new(InfixExpression {
                    operator: op.to_string(),
                    left: lhs,
                    right: rhs,
                })),
                _ => Err("Invalid combination"),
            }
          }
    )(input)
}

// =====

#[derive(Debug)]
pub struct IdentifierLiteral {
    name: String
}

impl Expression for IdentifierLiteral {}

#[derive(Debug)]
pub struct IntegerLiteral {
    value: String,
    length: String,
}

impl Expression for IntegerLiteral {}

#[derive(Debug)]
pub struct BooleanLiteral {
    value: bool
}

impl Expression for BooleanLiteral {}

// =====

#[derive(Debug)]
pub struct PrefixExpression {
    operator: String,
    right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {}

#[derive(Debug)]
pub struct InfixExpression {
    operator: String,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for InfixExpression {}

// =====

#[derive(Debug)]
pub struct LetStatement {
    name: IdentifierLiteral,
    mutable: bool,
    value: Box<dyn Expression>
}

impl Statement for LetStatement {}

#[derive(Debug)]
pub struct ReturnStatement {
    value: Box<dyn Expression>
}

impl Statement for ReturnStatement {}

// =====

#[derive(Debug)]
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

// =====

trait Expression: Debug {
    
}

trait Statement: Debug {
    
}