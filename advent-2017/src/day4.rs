use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .filter(|line| line.iter().combinations(2).all(|pair| pair[0] != pair[1]))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .filter(|line| {
            line.iter()
                .combinations(2)
                .all(|pair| pair[0].chars().sorted().ne(pair[1].chars().sorted()))
        })
        .count()
}
