use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

const INITIAL_CODE: usize = 20_151_125;
const MULTIPLIER: usize = 252_533;
const MODULUS: usize = 33_554_393;

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
    (1..target).fold(INITIAL_CODE, |code, _| (code * MULTIPLIER) % MODULUS)
}
