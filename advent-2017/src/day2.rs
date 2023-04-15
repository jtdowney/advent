use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn generator(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse()).collect())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|row| {
            let (min, max) = row.iter().minmax().into_option().unwrap();
            max - min
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|row| {
            row.iter()
                .permutations(2)
                .find(|p| p[0] % p[1] == 0)
                .map(|p| p[0] / p[1])
                .unwrap_or_default()
        })
        .sum()
}
