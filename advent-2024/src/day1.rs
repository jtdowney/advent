use anyhow::Context;
use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generator(input: &str) -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    input
        .lines()
        .try_fold((vec![], vec![]), |(mut left, mut right), line| {
            let mut parts = line.split_whitespace();
            let a = parts.next().context("missing column")?.parse()?;
            let b = parts.next().context("missing column")?.parse()?;
            left.push(a);
            right.push(b);
            Ok((left, right))
        })
}

#[aoc(day1, part1)]
fn part1((left, right): &(Vec<i32>, Vec<i32>)) -> u32 {
    left.iter()
        .sorted_unstable()
        .zip(right.iter().sorted_unstable().copied())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
fn part2((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    let right = right.iter().counts();

    left.iter()
        .map(|&a| {
            let b = right.get(&a).copied().unwrap_or(0) as i32;
            a * b
        })
        .sum()
}
