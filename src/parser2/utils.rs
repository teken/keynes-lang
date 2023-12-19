use nom::{IResult, bytes::complete::{take_while, take_while1}};


pub fn parse_number(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_numeric() || c == '_')(input)
}

pub fn parse_number1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_numeric() || c == '_')(input)
}

pub fn parse_identifier1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}