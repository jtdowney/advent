use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn generator(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .context("parsing numbers")
        })
        .collect()
}

fn expand(values: &[i32], reverse: bool) -> Option<i32> {
    if values.iter().all(|&v| v == 0) {
        return Some(0);
    }

    let next_level = values
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    let expanded = expand(&next_level, reverse)?;

    if reverse {
        let value = values.first()?;
        Some(value - expanded)
    } else {
        let value = values.last()?;
        Some(value + expanded)
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> Option<i32> {
    input.iter().map(|values| expand(values, false)).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> Option<i32> {
    input.iter().map(|values| expand(values, true)).sum()
}
