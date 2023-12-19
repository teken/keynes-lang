use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    sequence::delimited,
    sequence::tuple, multi::many0
};

use crate::ast2::{traits::*, statements::*};

use super::{expressions::*, literals::*};

pub fn parse_statment(input: &str) -> IResult<&str, Box<dyn Statement>> {
    delimited(multispace0, alt((
        parse_let_statement,
        parse_return_statement,
        parse_expression_statement
    )), multispace0)(input)
}

pub fn parse_let_statement(input: &str) -> IResult<&str, Box<dyn Statement>> {
    tuple((
        multispace0,
        tag("let"),
        multispace1,
        opt(tag("mut")),
        multispace0,
        parse_identifier_literal,
        multispace0,
        tag("="),
        multispace0,
        parse_expression,
        multispace0,
        tag(";"),
        multispace0
    ))(input).map(|(input, (_,_, _, mutable, _, name, _, _, _, value, _, _, _))| {   
        (input, Box::new(LetStatement { mutable: mutable.is_some(), name, value }) as Box<dyn Statement>)
    })
}

pub fn parse_return_statement(input: &str) -> IResult<&str, Box<dyn Statement>> {
    tuple((
        multispace0,
        tag("return"),
        multispace1,
        parse_expression,
        multispace0,
        tag(";"),
    ))(input).map(|(input, (_, _, _, value, _, _))| {   
        (input, Box::new(ReturnStatement { value }) as Box<dyn Statement>)
    })
}

pub fn parse_expression_statement(input: &str) -> IResult<&str, Box<dyn Statement>> {
    tuple((
        multispace0,
        parse_expression,
        multispace0,
        tag(";"),
    ))(input).map(|(input, (_, expression, _, _))| {   
        (input, Box::new(ExpressionStatement { expression }) as Box<dyn Statement>)
    })
}

pub fn parse_block_statement(input: &str) -> IResult<&str, BlockStatement> {
    tuple((
        multispace0,
        tag("{"),
        multispace0,
        many0(parse_statment),
        multispace0,
        tag("}"),
        multispace0
    ))(input).map(|(input, (_, _, _, statements, _, _, _))| {   
        (input, BlockStatement { statements })
    })
}