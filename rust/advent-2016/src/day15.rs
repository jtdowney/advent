use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone, Copy)]
struct Disc {
    positions: usize,
    start: usize,
}

#[aoc_generator(day15)]
fn generator(input: &str) -> anyhow::Result<Vec<Disc>> {
    let regex = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).")?;
    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).context("unable to match line")?;
            let positions = captures[1].parse()?;
            let start = captures[2].parse()?;
            Ok(Disc { positions, start })
        })
        .collect()
}

#[aoc(day15, part1)]
fn part1(input: &[Disc]) -> Option<usize> {
    (0..).find(|time| {
        input
            .iter()
            .enumerate()
            .all(|(i, disc)| (disc.start + time + i + 1) % disc.positions == 0)
    })
}

#[aoc(day15, part2)]
fn part2(input: &[Disc]) -> Option<usize> {
    let mut discs = input.to_vec();
    discs.push(Disc {
        positions: 11,
        start: 0,
    });

    (0..).find(|time| {
        discs
            .iter()
            .enumerate()
            .all(|(i, disc)| (disc.start + time + i + 1) % disc.positions == 0)
    })
}
