use std::str::FromStr;

use crate::lld::{Block, Memory};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, multispace0, multispace1, space0},
    combinator::{map, map_res, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult, Parser,
};

use super::num::parse_int;

// SP*[name]SP*
// NOTE: now, [name] should be alphanumeric. should modify the last space0 to space1?
fn name(i: &str) -> IResult<&str, String> {
    map_res(delimited(space0, alphanumeric1, space0), String::from_str)(i)
}

// SP*[attr]?SP*
fn attr(i: &str) -> IResult<&str, String> {
    map_res(
        tuple((
            space0,
            delimited(tag("("), take_until(")"), tag(")")),
            space0,
        )),
        |(_, attr, _)| String::from_str(str::trim(attr)),
    )(i)
}

// SP*("ORIGIN"|"org"|"o")SP*
fn origin(i: &str) -> IResult<&str, &str> {
    map(
        tuple((space0, alt((tag("ORIGIN"), tag("org"), tag("o"))), space0)),
        |(_, origin, _)| origin,
    )(i)
}

// SP*("LENGTH"|"len"|"l")SP*
fn length(i: &str) -> IResult<&str, &str> {
    map(
        tuple((space0, alt((tag("LENGTH"), tag("len"), tag("l"))), space0)),
        |(_, origin, _)| origin,
    )(i)
}

// [name] ([attr])? : ORIGIN = [origin], LENGTH = [len]
fn block(i: &str) -> IResult<&str, Block> {
    map(
        tuple((
            name,
            opt(attr),
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

// SP* MEMORY SP* { (NL* [block])? (NL+ [block])* NL* }
pub fn memory(i: &str) -> IResult<&str, Memory> {
    let block_lines = separated_list0(multispace1, block);
    delimited(
        tuple((
            multispace0,
            tag("MEMORY"),
            multispace0,
            tag("{"),
            multispace0,
        )),
        block_lines,
        tuple((multispace0, tag("}"))),
    )(i)
    .map(|(r, blocks)| (r, Memory { blocks }))
}

// fn block(i: &str) -> IResult<&str, Memory> {
//     preceded(sp, second)
// }

#[test]
fn memory_test() {
    let cases = [
        "MEMORY
  {
    rom (rx)  : ORIGIN = 0, LENGTH = 256K
    ram (!rx) : org = 0x40000000, l = 4M
  }",
        "MEMORY
  {
    rom (rx)  : ORIGIN = 0, LENGTH = 256K

    ram (!rx) : org = 0x40000000, l = 4M



  }",
        "MEMORY{rom (rx)  : ORIGIN = 0, LENGTH = 256K}",
        "MEMORY{
rom (rx)  : ORIGIN = 0, LENGTH = 256K   }",
    ];
    for case in cases {
        assert!(memory(case).is_ok());
    }

    let bad_cases = [
        "MEMORY
  {
    rom (rx)  : ORIGIN = 0, LENGTH = 256K  ram (!rx) : org = 0x40000000, l = 4M
  }",
        "MEMORY{rom (rx)  : ORIGIN = 0, 
LENGTH = 256K}",
    ];
    for case in bad_cases {
        assert!(!memory(case).is_ok());
    }
}
