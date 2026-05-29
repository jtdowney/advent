use anyhow::Context;
use itertools::Itertools;

const SEARCH_VALUE: u32 = 2020;

#[aoc_generator(day1)]
fn generator(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .with_context(|| format!("Failed to parse line: '{}'", line))
        })
        .collect()
}

fn solve(size: usize, input: &[u32]) -> anyhow::Result<u32> {
    let entries = input
        .iter()
        .combinations(size)
        .find(|entries| entries.iter().copied().sum::<u32>() == SEARCH_VALUE)
        .with_context(|| {
            format!(
                "No combination of {} numbers found that sum to {}",
                size, SEARCH_VALUE
            )
        })?;

    Ok(entries.iter().copied().product())
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> anyhow::Result<u32> {
    solve(2, input)
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> anyhow::Result<u32> {
    solve(3, input)
}
