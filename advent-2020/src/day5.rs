use std::str::FromStr;

use anyhow::{Context, Result, bail};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct BoardingPass(u16);

impl FromStr for BoardingPass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .chars()
            .enumerate()
            .map(|(i, ch)| match ch {
                'F' => Ok('0'),
                'B' => Ok('1'),
                'L' => Ok('0'),
                'R' => Ok('1'),
                _ => bail!(
                    "Invalid character '{}' at position {} in boarding pass: '{}'",
                    ch,
                    i,
                    s
                ),
            })
            .collect::<Result<String>>()?;
        u16::from_str_radix(&id, 2)
            .map(BoardingPass)
            .with_context(|| {
                format!(
                    "Failed to parse boarding pass ID from binary string: '{}'",
                    id
                )
            })
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Result<Vec<BoardingPass>> {
    input
        .lines()
        .map(|line| {
            line.parse::<BoardingPass>()
                .with_context(|| format!("Failed to parse boarding pass: '{}'", line))
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(passes: &[BoardingPass]) -> Result<u16> {
    passes
        .iter()
        .max()
        .map(|&BoardingPass(n)| n)
        .context("No boarding passes found")
}

#[aoc(day5, part2)]
fn part2(passes: &[BoardingPass]) -> Result<u16> {
    let mut passes = passes.to_vec();
    passes.sort_unstable();

    let neighbors = passes
        .windows(2)
        .find(|parts| {
            let BoardingPass(a) = parts[0];
            let BoardingPass(b) = parts[1];
            b - a == 2
        })
        .map(|passes| passes[0].0)
        .context("No gap found in boarding passes")?;

    Ok(neighbors + 1)
}
