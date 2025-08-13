use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::char,
    combinator::{map, map_res, value, verify},
    multi::fold_many0,
    sequence::{delimited, preceded},
};

#[derive(Debug, Clone, Copy)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(u8),
}

fn hex_encoded(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("x").parse(input)?;
    map_res(
        take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()),
        |value| u8::from_str_radix(value, 16),
    )
    .parse(input)
}

fn escaped(input: &str) -> IResult<&str, u8> {
    preceded(
        char('\\'),
        alt((
            hex_encoded,
            value(b'"', char('"')),
            value(b'\\', char('\\')),
        )),
    )
    .parse(input)
}

fn literal(input: &str) -> IResult<&str, &str> {
    let not_quote_slash = is_not("\"\\");
    verify(not_quote_slash, |s: &str| !s.is_empty()).parse(input)
}

fn fragment(input: &str) -> IResult<&str, StringFragment<'_>> {
    alt((
        map(literal, StringFragment::Literal),
        map(escaped, StringFragment::EscapedChar),
    ))
    .parse(input)
}

fn string(input: &str) -> IResult<&str, Vec<u8>> {
    delimited(
        tag("\""),
        fold_many0(fragment, Vec::new, |mut acc, fragment| {
            match fragment {
                StringFragment::Literal(s) => acc.extend_from_slice(s.as_bytes()),
                StringFragment::EscapedChar(c) => acc.push(c),
            }
            acc
        }),
        tag("\""),
    )
    .parse(input)
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

#[aoc(day8, part1)]
fn part1(lines: &[String]) -> anyhow::Result<usize> {
    lines
        .iter()
        .map(|line| {
            let parsed_length = match string(line) {
                Ok((_, data)) => data.len(),
                Err(e) => {
                    bail!("unable to parse {line}: {e}");
                }
            };

            Ok(line.len() - parsed_length)
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| {
            let mut expanded = String::new();
            expanded.push('\"');

            let mut position = 0;
            let bytes = line.as_bytes();
            while position < bytes.len() {
                match bytes[position] {
                    b'"' => {
                        expanded.push_str("\\\"");
                        position += 1;
                    }
                    b'\\' => {
                        expanded.push_str("\\\\");
                        position += 1;

                        match bytes[position] {
                            b'\\' => {
                                expanded.push_str("\\\\");
                                position += 1;
                            }
                            b'"' => {
                                expanded.push_str("\\\"");
                                position += 1;
                            }
                            b'x' => {
                                for b in bytes[position..].iter().take(3) {
                                    expanded.push(*b as char);
                                }

                                position += 3;
                            }
                            _ => unreachable!(),
                        }
                    }
                    b => {
                        expanded.push(b as char);
                        position += 1;
                    }
                }
            }

            expanded.push('\"');
            expanded.len() - line.len()
        })
        .sum()
}
