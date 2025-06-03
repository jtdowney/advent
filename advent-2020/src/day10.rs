use std::collections::HashMap;

use anyhow::{Context, Result};

#[aoc_generator(day10)]
fn generator(input: &str) -> Result<Vec<usize>> {
    let mut numbers = input
        .lines()
        .map(|line| {
            line.parse::<usize>()
                .with_context(|| format!("Failed to parse number: '{}'", line))
        })
        .collect::<Result<Vec<_>>>()?;
    numbers.sort_unstable();
    Ok(numbers)
}

#[aoc(day10, part1)]
fn part1(input: &[usize]) -> usize {
    let (ones, threes) = input
        .iter()
        .scan(0, |prev, &value| {
            let diff = value - *prev;
            *prev = value;

            Some(diff)
        })
        .fold((0, 1), |(mut ones, mut threes), difference| {
            match difference {
                1 => ones += 1,
                3 => threes += 1,
                _ => {}
            }

            (ones, threes)
        });

    ones * threes
}

#[aoc(day10, part2)]
fn part2(input: &[usize]) -> Result<u64> {
    let goal = input.last().context("Input is empty")?;
    let mut cache = HashMap::new();

    for &i in input {
        cache.insert(i, 0);

        if i <= 3 {
            cache.entry(i).and_modify(|v| *v += 1);
        }

        let end = i.min(3);
        let total = (1..=end)
            .filter_map(|n| {
                let index = i - n;
                cache.get(&index)
            })
            .sum::<u64>();
        cache.entry(i).and_modify(|v| *v += total);
    }

    cache
        .get(goal)
        .copied()
        .context("Goal adapter not found in cache")
}
