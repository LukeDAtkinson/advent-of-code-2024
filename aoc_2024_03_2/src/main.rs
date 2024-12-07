use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file contents to string");

    let (_, results) = parse_line(&input).unwrap();
    println!("{:?}", results);
    let mut active = true;
    let mut result = 0;
    for r in results {
        match r {
            ParseResult::Do => active = true,
            ParseResult::Dont => active = false,
            ParseResult::Mul(a) => {
                if active {
                    result += a
                }
            }
            ParseResult::None => {}
        }
    }

    println!("{}", result);
}

fn parse_line(line: &str) -> IResult<&str, Vec<ParseResult>> {
    many0(alt((parse_mul, parse_do, parse_dont, parse_none)))(line)
}

fn parse_mul(line: &str) -> IResult<&str, ParseResult> {
    let (remaining, (a, b)) =
        delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")"))(line)?;
    Ok((remaining, ParseResult::Mul(a * b)))
}

fn parse_do(line: &str) -> IResult<&str, ParseResult> {
    let (remaining, _) =
        tag("do()")(line)?;
    Ok((remaining, ParseResult::Do))
}
fn parse_dont(line: &str) -> IResult<&str, ParseResult> {
    let (remaining, _) =
        tag("don't()")(line)?;
    Ok((remaining, ParseResult::Dont))
}
fn parse_none(line: &str) -> IResult<&str, ParseResult> {
    let (remaining, _) = anychar(line)?;
    Ok((remaining, ParseResult::None))
}

#[derive(Debug)]
enum ParseResult {
    Do,
    Dont,
    Mul(u32),
    None,
}
