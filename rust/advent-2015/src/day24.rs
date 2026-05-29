use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day24)]
fn generator(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

fn solve(packages: &[u64], size: u64) -> u64 {
    let sum = packages.iter().sum::<u64>() / size;
    let mut min = packages.len();

    for i in 1..packages.len() {
        if packages
            .iter()
            .combinations(i)
            .any(|combination| combination.iter().copied().sum::<u64>() == sum)
        {
            min = i;
            break;
        }
    }

    packages
        .iter()
        .combinations(min)
        .filter(|combination| combination.iter().copied().sum::<u64>() == sum)
        .map(|combination| combination.iter().copied().product())
        .min()
        .unwrap()
}

#[aoc(day24, part1)]
fn part1(input: &[u64]) -> u64 {
    solve(input, 3)
}

#[aoc(day24, part2)]
fn part2(input: &[u64]) -> u64 {
    solve(input, 4)
}
