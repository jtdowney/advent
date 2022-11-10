use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::ContextCompat;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<start>.+) to (?P<end>.+) = (?P<distance>\d+)$").unwrap());

#[derive(Default)]
struct Input {
    locations: HashSet<String>,
    distances: HashMap<(String, String), usize>,
}

impl FromStr for Input {
    type Err = eyre::Report;

    fn from_str(s: &str) -> eyre::Result<Self> {
        s.lines().try_fold(Input::default(), |mut acc, line| {
            let captures = REGEX.captures(line).context("unable to match line: {s}")?;
            let start = captures.name("start").unwrap().as_str().to_string();
            let end = captures.name("end").unwrap().as_str().to_string();
            let distance = captures
                .name("distance")
                .and_then(|v| v.as_str().parse().ok())
                .unwrap();
            acc.locations.insert(start.clone());
            acc.locations.insert(end.clone());
            acc.distances.insert((start, end), distance);
            Ok(acc)
        })
    }
}

#[aoc_generator(day9)]
fn generator(input: &str) -> eyre::Result<Input> {
    input.parse()
}

fn distances(input: &Input) -> impl Iterator<Item = usize> + '_ {
    input
        .locations
        .iter()
        .permutations(input.locations.len())
        .filter_map(|route| {
            route
                .iter()
                .tuple_windows()
                .map(|(first, second)| {
                    input
                        .distances
                        .get(&(first.to_string(), second.to_string()))
                        .or_else(|| {
                            input
                                .distances
                                .get(&(second.to_string(), first.to_string()))
                        })
                })
                .sum()
        })
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> usize {
    distances(input).min().unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> usize {
    distances(input).max().unwrap()
}
