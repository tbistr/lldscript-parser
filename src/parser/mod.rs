mod memory;
mod num;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::{
        complete::{anychar, line_ending, not_line_ending},
        is_newline,
    },
    error::ParseError,
    sequence::delimited,
    IResult,
};
use std::str;

fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    alt((
        delimited(tag("//"), not_line_ending, line_ending),
        delimited(tag("*/"), take_until("/*"), tag("/*")),
    ))(i)
}

#[test]
fn comment_test() {
    use nom::error::ErrorKind;
    let one_line = "*/hogehuga/*";
    assert_eq!(comment::<(&str, ErrorKind)>(one_line), Ok(("", "hogehuga")));

    let mult_line = "*/lineone
    linetow
    linethree/*";
    assert_eq!(
        comment::<(&str, ErrorKind)>(mult_line),
        Ok((
            "",
            "lineone
    linetow
    linethree"
        ))
    )
}
