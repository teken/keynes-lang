use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    sequence::{delimited, separated_pair},
};

use crate::ast2::literals::*;

use super::utils::*;

pub fn parse_identifier_literal(input: &str) -> IResult<&str, IdentifierLiteral> {
    delimited(multispace0, parse_identifier1, multispace0
    )(input).map(|(input, name)| {
        (input, IdentifierLiteral { name: name.to_string() })
    })
}

pub fn parse_integer_literal(input: &str) -> IResult<&str, IntegerLiteral> {
    delimited(multispace0, separated_pair(
        parse_number1,
        tag("i"),
        parse_number,
    ), multispace0)(input).map(|(input, (value, length))| {
        (input, IntegerLiteral {
            value: value.to_string(),
            length: length.to_string(),
        })
    })
}

pub fn parse_float_literal(input: &str) -> IResult<&str, FloatLiteral> {
    delimited(multispace0, separated_pair(
        parse_number1,
        tag("f"),
        parse_number1,
    ), multispace0)(input).map(|(input, (value, length))| {
        (input, FloatLiteral {
            value: value.to_string(),
            length: length.to_string(),
        })
    })
}

pub fn parse_boolean_literal(input: &str) -> IResult<&str, BooleanLiteral> {
    delimited(multispace0, alt((
        map(tag("true"), |_| BooleanLiteral { value: true }),
        map(tag("false"), |_| BooleanLiteral { value: false }),
    )), multispace0)(input)
}