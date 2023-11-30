use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::ContextCompat;

type Assignments = HashSet<u32>;

fn parse_assignments(part: &str) -> Option<Assignments> {
    let mut range_parts = part.split('-');
    let start = range_parts.next().and_then(|n| n.parse().ok())?;
    let end = range_parts.next().and_then(|n| n.parse().ok())?;
    let set = (start..=end).collect();
    Some(set)
}

#[aoc_generator(day4)]
fn generator(input: &str) -> eyre::Result<Vec<(Assignments, Assignments)>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let left = parts.next().and_then(parse_assignments)?;
            let right = parts.next().and_then(parse_assignments)?;
            Some((left, right))
        })
        .collect::<Option<Vec<_>>>()
        .context("unable to parse input")
}

#[aoc(day4, part1)]
fn part1(input: &[(Assignments, Assignments)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| left.is_subset(right) || right.is_subset(left))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[(Assignments, Assignments)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| !left.is_disjoint(right))
        .count()
}
