use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generator(input: &str) -> anyhow::Result<Vec<Vec<usize>>> {
    input.lines().try_fold(vec![vec![]], |mut acc, line| {
        if line.is_empty() {
            acc.push(vec![]);
            return Ok(acc);
        }

        let current = acc.last_mut().context("unable to get last element")?;
        let value = line.parse()?;
        current.push(value);

        Ok(acc)
    })
}

#[aoc(day1, part1)]
fn part1(input: &[Vec<usize>]) -> Option<usize> {
    input
        .iter()
        .map(|values| values.iter().sum::<usize>())
        .max()
}

#[aoc(day1, part2)]
fn part2(input: &[Vec<usize>]) -> usize {
    let mut totals = input
        .iter()
        .map(|values| values.iter().sum::<usize>())
        .collect::<Vec<usize>>();
    totals.sort_unstable();
    totals.iter().rev().take(3).sum()
}
