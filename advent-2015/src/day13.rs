use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<actor>.+) would (?P<action>gain|lose) (?P<units>\d+) happiness units by sitting next to (?P<target>.+).$").unwrap()
});

#[derive(Clone, Debug, Default)]
struct Input {
    actors: HashSet<String>,
    happiness: HashMap<(String, String), isize>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        s.lines().try_fold(Input::default(), |mut acc, line| {
            let captures = REGEX.captures(line).context("unable to match line: {s}")?;
            let actor = captures.name("actor").unwrap().as_str().to_string();
            let action = captures.name("action").unwrap().as_str();
            let target = captures.name("target").unwrap().as_str().to_string();
            let mut units: isize = captures
                .name("units")
                .and_then(|v| v.as_str().parse().ok())
                .unwrap();

            if action == "lose" {
                units = -units;
            }

            acc.actors.insert(actor.clone());
            acc.happiness.insert((actor, target), units);
            Ok(acc)
        })
    }
}

#[aoc_generator(day13)]
fn generator(input: &str) -> anyhow::Result<Input> {
    input.parse()
}

fn solve(input: &Input) -> isize {
    input
        .actors
        .iter()
        .cloned()
        .permutations(input.actors.len())
        .map(|mut seats| {
            let forward = seats
                .iter()
                .tuple_windows()
                .filter_map(|(left, right)| {
                    input.happiness.get(&(left.to_string(), right.to_string()))
                })
                .sum::<isize>();
            let backward = seats
                .iter()
                .rev()
                .tuple_windows()
                .filter_map(|(left, right)| {
                    input.happiness.get(&(left.to_string(), right.to_string()))
                })
                .sum::<isize>();

            let last = seats.pop().unwrap();
            let first = seats.remove(0);

            let wrap_forward = input.happiness[&(last.clone(), first.clone())];
            let wrap_backward = input.happiness[&(first, last)];

            forward + backward + wrap_forward + wrap_backward
        })
        .max()
        .unwrap()
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> isize {
    solve(input)
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> isize {
    let mut input = input.clone();
    input.actors.insert("Self".to_string());
    for actor in &input.actors {
        input
            .happiness
            .insert(("Self".to_string(), actor.clone()), 0);
        input
            .happiness
            .insert((actor.clone(), "Self".to_string()), 0);
    }

    solve(&input)
}
