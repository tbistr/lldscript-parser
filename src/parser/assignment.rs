use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, one_of},
    combinator::recognize,
    multi::many0,
    sequence::pair,
    IResult,
};

struct Assignment {
    symbol: String,
    op: Operator,
    expression: String,
}

enum Operator {}
fn symbol(i: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"), tag("."))),
        many0(alt((alphanumeric1, tag("_"), tag("."), tag("-")))),
    ))(i)
}
