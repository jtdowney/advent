use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day25)]
fn generator(input: &str) -> anyhow::Result<(usize, usize)> {
    let regex = Regex::new(r"row (\d+), column (\d+)")?;
    let captures = regex.captures(input).context("Invalid input")?;
    let row = captures[1].parse()?;
    let column = captures[2].parse()?;
    Ok((row, column))
}

#[aoc(day25, part1)]
fn part1(&(row, column): &(usize, usize)) -> usize {
    let side = row + column - 1;
    let target = side * (side + 1) / 2 - row + 1;
    (1..target).fold(20151125, |code, _| (code * 252533) % 33554393)
}
