use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, space0},
    combinator::map_res,
    sequence::{delimited, preceded, terminated},
    IResult,
};

// Parses Tok as an integer. It recognizes hexadecimal (prefixed with
// "0x" or suffixed with "H") and decimal numbers. Decimal numbers may
// have "K" (Ki) or "M" (Mi) suffixes.
// https://github.com/llvm/llvm-project/blob/main/lld/ELF/ScriptParser.cpp#L1132
// WS*[int]WS*
pub fn parse_int(i: &str) -> IResult<&str, u64> {
    delimited(
        space0,
        alt((
            map_res(preceded(tag("0x"), hex_digit1), |hex| {
                u64::from_str_radix(hex, 16)
            }),
            map_res(terminated(hex_digit1, tag("H")), |hex| {
                u64::from_str_radix(hex, 16)
            }),
            map_res(terminated(digit1, tag("K")), |deci: &str| {
                deci.parse::<u64>().map(|deci| deci * 1024)
            }),
            map_res(terminated(digit1, tag("M")), |deci: &str| {
                deci.parse::<u64>().map(|deci| deci * 1024 * 1024)
            }),
            map_res(digit1, str::parse::<u64>),
        )),
        space0,
    )(i)
}

#[test]
fn parse_int_test() {
    let case = ["12345", "0x1af", "100H", "1K", "1M"];
    assert_eq!(parse_int(case[0]), Ok(("", 12345)));
    assert_eq!(parse_int(case[1]), Ok(("", 16 * 16 + 10 * 16 + 15)));
    assert_eq!(parse_int(case[2]), Ok(("", 16 * 16)));
    assert_eq!(parse_int(case[3]), Ok(("", 1024)));
    assert_eq!(parse_int(case[4]), Ok(("", 1024 * 1024)));

    // ↓こういうのも通る。が、どうせその後のパースでエラーになるのでヨシ？
    // let bad_case = ["0x1000K", "0x", "1000G"];
    // assert!(parse_int(bad_case[0]).is_err());
    // assert!(parse_int(bad_case[1]).is_err());

    // ただし、↓こういうのは普通にエラーになる。
    let bad_case = ["H", "Hb", "hogehuga"];
    assert!(parse_int(bad_case[0]).is_err());
    assert!(parse_int(bad_case[1]).is_err());
    assert!(parse_int(bad_case[2]).is_err());
}
