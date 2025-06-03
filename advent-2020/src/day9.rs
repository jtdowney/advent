use anyhow::{Context, Result};
use itertools::Itertools;

const PREAMBLE_SIZE: usize = 25;

#[aoc_generator(day9)]
fn generator(input: &str) -> Result<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .with_context(|| format!("Failed to parse number: '{}'", line))
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[u64]) -> Result<u64> {
    input
        .windows(PREAMBLE_SIZE + 1)
        .find_map(|window| {
            let sum = window[PREAMBLE_SIZE];
            let found = window[0..PREAMBLE_SIZE]
                .iter()
                .combinations(2)
                .any(|numbers| numbers.iter().copied().sum::<u64>() == sum);
            if found { None } else { Some(sum) }
        })
        .context("No invalid number found")
}

#[aoc(day9, part2)]
fn part2(input: &[u64]) -> Result<u64> {
    let sum = part1(input)?;
    let answer = (2..input.len())
        .map(|n| input.windows(n))
        .find_map(|mut windows| windows.find(|window| window.iter().copied().sum::<u64>() == sum))
        .context("No contiguous range found")?;

    let (min, max) = answer
        .iter()
        .copied()
        .minmax()
        .into_option()
        .context("Failed to find min/max in range")?;
    Ok(min + max)
}
