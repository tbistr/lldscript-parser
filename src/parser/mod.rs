mod memory;
mod num;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending},
    sequence::{delimited, tuple},
    IResult,
};
use std::str;

use crate::lld;

pub fn parse(i: &str) -> Result<lld::Script, String> {
    tuple((take_until("MEMORY"), memory::memory))(i)
        .map(|(suf, (pre, memory))| lld::Script {
            others1: pre.to_string(),
            memory,
            others2: suf.to_string(),
        })
        .map_err(|e| e.to_string())
}

fn comment(i: &str) -> IResult<&str, &str> {
    alt((
        delimited(tag("//"), not_line_ending, line_ending),
        delimited(tag("*/"), take_until("/*"), tag("/*")),
    ))(i)
}

#[test]
fn comment_test() {
    let one_line = "*/hogehuga/*";
    assert_eq!(comment(one_line), Ok(("", "hogehuga")));

    let mult_line = "*/lineone
    linetow
    linethree/*";
    assert_eq!(
        comment(mult_line),
        Ok((
            "",
            "lineone
    linetow
    linethree"
        ))
    )
}
