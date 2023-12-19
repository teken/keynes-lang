use super::statements::*;

use nom::{
    IResult,
    multi::many1,
};

use crate::ast2::program::Program;

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    many1(parse_statment)(input).map(|(input, statements)| {
        (input, Program {
            statements
        })
    })
}