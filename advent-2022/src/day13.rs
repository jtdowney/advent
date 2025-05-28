use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending, u32},
    combinator::{complete, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(lhs), Packet::Value(rhs)) => lhs.cmp(rhs),
            (Packet::List(lhs), Packet::List(rhs)) => lhs.cmp(rhs),
            (Packet::Value(lhs), Packet::List(rhs)) => vec![Packet::Value(*lhs)].cmp(rhs),
            (Packet::List(lhs), Packet::Value(rhs)) => lhs.cmp(&vec![Packet::Value(*rhs)]),
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let parse_value = map(u32, Packet::Value);
    map(
        delimited(
            char('['),
            separated_list0(char(','), alt((parse_value, parse_packet))),
            char(']'),
        ),
        Packet::List,
    )
    .parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    let parse_pair = separated_pair(parse_packet, line_ending, parse_packet);
    complete(separated_list1((line_ending, line_ending), parse_pair)).parse(input)
}

#[aoc_generator(day13)]
fn generator(input: &str) -> eyre::Result<Vec<(Packet, Packet)>> {
    match parse_input(input) {
        Ok((_, i)) => Ok(i),
        Err(e) => bail!("error parsing: {}", e),
    }
}

#[aoc(day13, part1)]
fn part1(input: &[(Packet, Packet)]) -> usize {
    input
        .iter()
        .zip(1..)
        .filter(|((left, right), _)| left < right)
        .map(|(_, i)| i)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[(Packet, Packet)]) -> usize {
    let dividers = vec![
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
    ];
    let input = input.to_vec();
    let (left, right): (Vec<Packet>, Vec<Packet>) = input.into_iter().unzip();
    let packets = left
        .into_iter()
        .chain(right)
        .chain(dividers.clone())
        .collect::<Vec<_>>();

    dividers
        .iter()
        .map(|divider| packets.iter().filter(|&packet| packet < divider).count() + 1)
        .product()
}
