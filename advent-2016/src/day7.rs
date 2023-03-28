use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, IResult};

#[derive(Debug)]
enum AddressPart {
    Normal(String),
    Hypernet(String),
}

fn is_abba(part: &str) -> bool {
    part.as_bytes()
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn aba_parts(part: &str) -> Vec<(u8, u8)> {
    part.as_bytes()
        .windows(3)
        .filter(|w| w[0] == w[2] && w[0] != w[1])
        .map(|w| (w[0], w[1]))
        .collect()
}

fn is_bab(part: &str, (a, b): (u8, u8)) -> bool {
    part.as_bytes()
        .windows(3)
        .any(|w| w[0] == b && w[1] == a && w[2] == b)
}

#[derive(Debug)]
struct Address {
    parts: Vec<AddressPart>,
}

impl Address {
    fn supports_tls(&self) -> bool {
        self.parts.iter().any(|part| {
            if let AddressPart::Normal(s) = part {
                is_abba(s)
            } else {
                false
            }
        }) && !self.parts.iter().any(|part| {
            if let AddressPart::Hypernet(s) = part {
                is_abba(s)
            } else {
                false
            }
        })
    }

    fn supports_ssl(&self) -> bool {
        self.parts
            .iter()
            .filter_map(|part| {
                if let AddressPart::Normal(s) = part {
                    Some(s)
                } else {
                    None
                }
            })
            .flat_map(|s| aba_parts(s))
            .any(|aba| {
                self.parts
                    .iter()
                    .filter_map(|part| {
                        if let AddressPart::Hypernet(s) = part {
                            Some(s)
                        } else {
                            None
                        }
                    })
                    .any(|s| is_bab(s, aba))
            })
    }
}

fn address_part(input: &str) -> IResult<&str, String> {
    use nom::{bytes::complete::take_while1, combinator::map};
    map(
        take_while1(|c: char| c.is_ascii_alphabetic() && !(c == '[' || c == ']')),
        String::from,
    )(input)
}

fn normal_part(input: &str) -> IResult<&str, AddressPart> {
    use nom::combinator::map;
    map(address_part, AddressPart::Normal)(input)
}

fn hypernet_part(input: &str) -> IResult<&str, AddressPart> {
    use nom::{bytes::complete::tag, combinator::map, sequence::delimited};
    map(
        delimited(tag("["), address_part, tag("]")),
        AddressPart::Hypernet,
    )(input)
}

fn address(input: &str) -> IResult<&str, Address> {
    use nom::{branch::alt, combinator::map, multi::many1};
    map(many1(alt((normal_part, hypernet_part))), |parts| Address {
        parts,
    })(input)
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<Vec<Address>> {
    input
        .lines()
        .map(|line| {
            address(line)
                .finish()
                .map(|(_, addr)| addr)
                .map_err(|_| anyhow!("Invalid address: {:?}", line))
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Address]) -> usize {
    input.iter().filter(|addr| addr.supports_tls()).count()
}

#[aoc(day7, part2)]
fn part2(input: &[Address]) -> usize {
    input.iter().filter(|addr| addr.supports_ssl()).count()
}
