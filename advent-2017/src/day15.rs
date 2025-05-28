use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

const REDUCER: u64 = 2147483647;

#[derive(Clone, Copy)]
struct Generator {
    previous: u64,
    factor: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = (self.previous * self.factor) % REDUCER;
        self.previous = value;
        Some(value)
    }
}

#[aoc_generator(day15)]
fn generator(input: &str) -> anyhow::Result<(Generator, Generator)> {
    let mut lines = input.lines();
    let seed_a = lines
        .next()
        .and_then(|line| line.split(' ').next_back())
        .context("unable to get value")?
        .parse::<u64>()?;
    let seed_b = lines
        .next()
        .and_then(|line| line.split(' ').next_back())
        .context("unable to get value")?
        .parse::<u64>()?;

    Ok((
        Generator {
            previous: seed_a,
            factor: 16807,
        },
        Generator {
            previous: seed_b,
            factor: 48271,
        },
    ))
}

#[aoc(day15, part1)]
fn part1(input: &(Generator, Generator)) -> usize {
    let (a, b) = *input;
    a.zip(b)
        .take(40_000_000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}

#[aoc(day15, part2)]
fn part2(input: &(Generator, Generator)) -> usize {
    let (a, b) = *input;
    a.filter(|a| a % 4 == 0)
        .zip(b.filter(|b| b % 8 == 0))
        .take(5_000_000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}
