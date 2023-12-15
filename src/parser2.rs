use std::fmt::Debug;

use nom::{
    branch::alt,
    IResult,
    combinator::{
        opt, map,
    },
    bytes::complete::{
        tag_no_case,
        tag,
        take_while1, take_while, take,
    },
    character::complete::{
        multispace0,
        multispace1,
    }, multi::many1
};

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
    let (input, _) = tag_no_case("let")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, mutable) = opt(tag_no_case("mut"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag_no_case("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, _) = tag_no_case(";")(input)?;
    Ok((input, Box::new(LetStatement 
        { mutable: mutable.is_some(), name, value })))
}

fn parse_return_statement(input: &str) -> IResult<&str, Box<dyn Statement>> {
    let (input, _) = tag_no_case("return")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value) = parse_expression(input)?;
    Ok((input, Box::new(ReturnStatement { value })))
}

fn parse_identifier(input: &str) -> IResult<&str, IdentifierLiteral> {
    let (input, name) = take_while1(|c: char| c.is_alphanumeric())(input)?;
    Ok((input, IdentifierLiteral { name: name.to_string() }))
}

fn parse_integer(input: &str) -> IResult<&str, IntegerLiteral> {
    let (input, value) = take_while1(|c: char| c.is_numeric() || c == '_')(input)?;
    let (input, _) = tag_no_case("i")(input)?;
    let (input, length) = take_while(|c: char| c.is_numeric() || c == '_')(input)?;
    Ok((input, IntegerLiteral {
        value: value.to_string(),
        length: length.to_string(),
    }))
}

fn parse_boolean(input: &str) -> IResult<&str, BooleanLiteral> {
    let (input, exp) = alt((
        map(tag("true"), |_| BooleanLiteral { value: true }),
        map(tag("false"), |_| BooleanLiteral { value: false })
    ))(input)?;
    Ok((input, exp))
}

fn parse_expression(input: &str) -> IResult<&str, Box<dyn Expression>> {
    let (input, exp) = alt((
        map(parse_integer, |e| Box::new(e) as Box<dyn Expression>),
        map(parse_boolean, |e| Box::new(e) as Box<dyn Expression>),
        map(parse_identifier, |e| Box::new(e) as Box<dyn Expression>),
    ))(input)?;
    Ok((input, exp))
}

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

#[derive(Debug)]
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

trait Expression: Debug {
    
}

trait Statement: Debug {
    
}