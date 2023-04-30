use std::{collections::HashSet, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generator(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .cycle()
        .try_fold((0, HashSet::new()), |(previous, mut seen), value| {
            let current = previous + value;
            if seen.insert(current) {
                Ok((current, seen))
            } else {
                Err(current)
            }
        })
        .unwrap_err()
}
