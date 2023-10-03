use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn generator(input: &str) -> anyhow::Result<(u32, u32)> {
    let numbers = input
        .split('-')
        .map(|s| s.parse::<u32>().context("parse number"))
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok((numbers[0], numbers[1]))
}

fn is_sorted(code: &str) -> bool {
    let mut sorted = code.as_bytes().to_vec();
    sorted.sort();
    sorted == code.as_bytes()
}

#[aoc(day4, part1)]
fn part1(&(min, max): &(u32, u32)) -> usize {
    (min..=max)
        .map(|code| code.to_string())
        .filter(|code| code.as_bytes().windows(2).any(|part| part[0] == part[1]))
        .filter(|code| is_sorted(code))
        .count()
}

#[aoc(day4, part2)]
fn part2(&(min, max): &(u32, u32)) -> usize {
    (min..=max)
        .map(|code| code.to_string())
        .filter(|code| {
            code.as_bytes()
                .windows(2)
                .any(|part| part[0] == part[1] && code.matches(part[0] as char).count() == 2)
        })
        .filter(|code| is_sorted(code))
        .count()
}
