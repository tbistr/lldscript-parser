use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, multispace0, oct_digit0, space0, space1},
    combinator,
    sequence::{self, delimited, preceded, tuple},
    IResult,
};

use super::{num::parse_int, sp};

// name [(attr)] : ORIGIN = origin, LENGTH = len
#[derive(Debug, PartialEq)]
struct Block {
    name: String,
    attr: Option<String>,
    origin: u64,
    length: u64,
}

type Memory = Vec<Block>;

// WS*[name]WS*
// NOTE: now, [name] should be alphanumeric. should modify the last space0 to space1?
fn name(i: &str) -> IResult<&str, String> {
    combinator::map_res(delimited(space0, alphanumeric1, space0), String::from_str)(i)
}

// WS*[attr]?WS*
fn attr(i: &str) -> IResult<&str, String> {
    combinator::map_res(
        sequence::tuple((
            space0,
            delimited(tag("("), take_until(")"), tag(")")),
            space0,
        )),
        |(_, attr, _)| String::from_str(str::trim(attr)),
    )(i)
}

// WS*("ORIGIN"|"org"|"o")WS*
fn origin(i: &str) -> IResult<&str, &str> {
    combinator::map(
        sequence::tuple((space0, alt((tag("ORIGIN"), tag("org"), tag("o"))), space0)),
        |(_, origin, _)| origin,
    )(i)
}

// WS*("LENGTH"|"len"|"l")WS*
fn length(i: &str) -> IResult<&str, &str> {
    combinator::map(
        sequence::tuple((space0, alt((tag("LENGTH"), tag("len"), tag("l"))), space0)),
        |(_, origin, _)| origin,
    )(i)
}

// [name] ([attr])? : ORIGIN = [origin], LENGTH = [len]
fn block(i: &str) -> IResult<&str, Block> {
    combinator::map(
        sequence::tuple((
            name,
            combinator::opt(attr),
            tag(":"),
            //
            origin,
            tag("="),
            parse_int,
            tag(","),
            //
            length,
            tag("="),
            parse_int,
        )),
        // name [(attr)] : ORIGIN = origin, LENGTH = len
        |(name, attr, _, _origin, _, origin, _, _length, _, length)| Block {
            name,
            attr,
            origin,
            length,
        },
    )(i)
}

#[test]
fn block_test() {
    let cases = [
        "ram (rw) : ORIGIN = 0x1000, LENGTH = 0x1000",
        "rom (!rw) : org = 1234, len = 1K",
        "ram (rw) : o = 1M, LENGTH = 1111H",
        "hoge   (  rw)  : ORIGIN   = 0x1000   , LENGTH   = 0x1000  ",
        "ram(rw) : ORIGIN = 0x1000, LENGTH = 0x1000", // TODO: check spec of the lld.
    ];
    assert_eq!(
        block(cases[0]),
        Ok((
            "",
            Block {
                name: "ram".to_string(),
                attr: Some("rw".to_string()),
                origin: 16 * 16 * 16,
                length: 16 * 16 * 16
            }
        ))
    );
    for case in cases {
        let res = block(case);
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}

fn memory(i: &str) -> IResult<&str, Memory> {
    let res: Memory = vec![];
    let (i, _) = tag("MEMORY")(i)?;
    let (i, _) = delimited(tag("{"), take_until("}"), tag("}"))(i)?;

    Ok(("", res))
}

// fn block(i: &str) -> IResult<&str, Memory> {
//     preceded(sp, second)
// }

#[test]
fn memory_test() {
    println!("{:?}", attr("(hogehuga)"))
}
