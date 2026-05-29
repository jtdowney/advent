use std::collections::HashSet;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn generator(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').context("missing colon")?;
            let (left, right) = numbers.split_once('|').context("missing separator")?;
            let winning_numbers = left
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<HashSet<usize>, _>>()?;
            let my_numbers = right
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<HashSet<_>, _>>()?;

            let count = (&winning_numbers & &my_numbers).len();
            Ok(count)
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .filter_map(|&count| {
            if count > 0 {
                Some(1 << (count - 1))
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[usize]) -> usize {
    let mut copies = vec![1; input.len()];
    for (i, &count) in input.iter().enumerate() {
        for offset in 1..=count {
            copies[i + offset] += copies[i];
        }
    }

    copies.iter().sum()
}
