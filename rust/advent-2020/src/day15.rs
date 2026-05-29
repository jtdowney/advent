use std::{collections::HashMap, iter};

use anyhow::{Context, Result};

fn solve(target_round: usize, seen: &HashMap<usize, usize>) -> Result<usize> {
    let mut seen = seen.clone();
    let target = target_round - seen.len();

    let mut round = seen.len();
    iter::successors(Some(0), |number| {
        let next = match seen.get(number) {
            Some(last_seen) => round - last_seen,
            None => 0,
        };

        seen.insert(*number, round);
        round += 1;

        Some(next)
    })
    .take(target)
    .last()
    .context("Failed to calculate final number")
}

#[aoc_generator(day15)]
fn generator(input: &str) -> Result<HashMap<usize, usize>> {
    input
        .split(',')
        .map(|s| {
            s.parse::<usize>()
                .with_context(|| format!("Failed to parse number: '{}'", s))
        })
        .enumerate()
        .map(|(i, r)| r.map(|s| (s, i)))
        .collect()
}

#[aoc(day15, part1)]
fn part1(seen: &HashMap<usize, usize>) -> Result<usize> {
    solve(2020, seen)
}

#[aoc(day15, part2)]
fn part2(seen: &HashMap<usize, usize>) -> Result<usize> {
    const TARGET_TURN: usize = 30_000_000;
    solve(TARGET_TURN, seen)
}
