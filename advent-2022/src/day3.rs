use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        u32::from(c) - 64 + 26
    } else {
        u32::from(c) - 96
    }
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|sack| {
            let chars = sack.chars().collect::<Vec<_>>();
            let (left, right) = chars.split_at(sack.len() / 2);

            let left = left.iter().collect::<HashSet<_>>();
            let right = right.iter().collect::<HashSet<_>>();
            left.intersection(&right)
                .map(|&c| priority(*c))
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> u32 {
    input
        .chunks(3)
        .map(|sacks| {
            let badges = sacks
                .iter()
                .map(|sack| sack.chars().collect::<HashSet<_>>())
                .reduce(|acc, item| &acc & &item);

            badges
                .iter()
                .map(|chars| chars.iter().map(|&c| priority(c)).sum::<u32>())
                .sum::<u32>()
        })
        .sum()
}
