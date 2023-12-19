use super::{literals::*, statements::parse_block_statement};

use nom::{
    branch::alt,
    IResult,
    combinator::{
        map, fail, opt, 
    },
    bytes::complete::tag,
    sequence::{delimited, tuple}, multi::many0,
};
use nom_7_precedence::{precedence, binary_op, Assoc, unary_op, Operation};

use crate::ast2::{traits::*, expressions::*};

pub fn parse_expression(input: &str) -> IResult<&str, Box<dyn Expression>> {
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
            binary_op(5, Assoc::Left, tag("..")),
            binary_op(5, Assoc::Left, tag("..=")),
        )),
        alt((
            delimited(tag("("), parse_expression, tag(")")), 
            parse_if_expression,
            map(parse_integer_literal, |i| Box::new(i) as Box<dyn Expression>),
            map(parse_float_literal, |i| Box::new(i) as Box<dyn Expression>),
            map(parse_boolean_literal, |b| Box::new(b) as Box<dyn Expression>),
            map(parse_identifier_literal, |i| Box::new(i) as Box<dyn Expression>),
       )),
        |op: Operation<&str, &str, &str, Box<dyn Expression>>| {
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

pub fn parse_if_expression(input: &str) -> IResult<&str, Box<dyn Expression>> {
    map(
        tuple((
            tag("if"),
            parse_expression,
            parse_block_statement,
            many0(tuple((
                tag("else if"),
                parse_expression,
                parse_block_statement,
            ))),
            opt(tuple((
                tag("else"),
                parse_block_statement,
            ))),
        )),
        |(_, condition, consequence, others, alternative)| {
            let mut exp = Box::new(IfExpression {
                conditions: vec![(condition, consequence)],
                alternative: alternative.map(|(_, alternative)| alternative)
            });

            for (_, condition1, consequence1) in others {
                exp.conditions.push((condition1, consequence1));
            }

            exp as Box<dyn Expression>
        },
    )(input)
}
