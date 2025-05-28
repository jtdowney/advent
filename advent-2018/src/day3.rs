use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use nom::{Finish, IResult, Parser};

#[derive(Clone, Copy, Debug)]
struct Claim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

fn claim(input: &str) -> IResult<&str, Claim> {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, u16},
        combinator::map,
        sequence::{preceded, separated_pair},
    };

    map(
        (
            preceded(char('#'), u16),
            tag(" @ "),
            separated_pair(u16, char(','), u16),
            tag(": "),
            separated_pair(u16, char('x'), u16),
        ),
        |(id, _, (x, y), _, (width, height))| Claim {
            id,
            x,
            y,
            width,
            height,
        },
    ).parse(input)
}

#[aoc_generator(day3)]
fn generator(input: &str) -> anyhow::Result<Vec<Claim>> {
    input
        .lines()
        .map(|line| {
            claim(line)
                .finish()
                .map(|(_, claim)| claim)
                .map_err(|e| anyhow!("Failed to parse claim: {:?}", e))
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Claim]) -> usize {
    input
        .iter()
        .fold(HashMap::new(), |mut acc, claim| {
            for point in iproduct!(
                claim.x..claim.x + claim.width,
                claim.y..claim.y + claim.height
            ) {
                *acc.entry(point).or_insert(0) += 1;
            }

            acc
        })
        .into_values()
        .filter(|&n| n > 1)
        .count()
}

#[aoc(day3, part2)]
fn part2(input: &[Claim]) -> Option<u16> {
    let repeated = input
        .iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, claim| {
            for point in iproduct!(
                claim.x..claim.x + claim.width,
                claim.y..claim.y + claim.height
            ) {
                acc.entry(point).or_default().push(claim.id);
            }

            acc
        })
        .into_values()
        .filter(|claims| claims.len() > 1)
        .flatten()
        .collect::<HashSet<u16>>();

    let all_claims = input.iter().map(|claim| claim.id).collect::<HashSet<u16>>();
    all_claims.difference(&repeated).next().copied()
}
