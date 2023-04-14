use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generator(input: &str) -> Option<Vec<u32>> {
    input.chars().map(|c| c.to_digit(10)).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input
        .iter()
        .enumerate()
        .filter(|&(i, n)| *n == input[(i + 1) % input.len()])
        .map(|(_, n)| n)
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    let half = input.len() / 2;
    input
        .iter()
        .enumerate()
        .filter(|&(i, n)| *n == input[(i + half) % input.len()])
        .map(|(_, n)| n)
        .sum()
}
