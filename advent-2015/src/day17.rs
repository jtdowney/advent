use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const REQUIRED_SIZE: usize = 150;

#[aoc_generator(day17)]
fn generator(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day17, part1)]
fn part1(input: &[usize]) -> usize {
    (1..input.len())
        .flat_map(|n| input.iter().combinations(n))
        .filter(|c| c.iter().copied().sum::<usize>() == REQUIRED_SIZE)
        .count()
}

#[aoc(day17, part2)]
fn part2(input: &[usize]) -> usize {
    let solutins = (1..input.len())
        .flat_map(|n| input.iter().combinations(n))
        .filter(|c| c.iter().copied().sum::<usize>() == REQUIRED_SIZE)
        .collect::<Vec<_>>();
    let min = solutins.iter().map(|c| c.len()).min().unwrap();

    solutins.iter().filter(|c| c.len() == min).count()
}
