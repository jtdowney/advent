use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::LazyLock,
};

use anyhow::{bail, Context};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<command>turn on|turn off|toggle) (?P<sx>\d+),(?P<sy>\d+) through (?P<ex>\d+),(?P<ey>\d+)$")
	.unwrap()
});

type Point = (usize, usize);

#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let command = match s {
            "turn on" => Command::TurnOn,
            "turn off" => Command::TurnOff,
            "toggle" => Command::Toggle,
            _ => bail!("unknown command: {s}"),
        };

        Ok(command)
    }
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    start: Point,
    end: Point,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let captures = REGEX.captures(s).context("unable to match line: {s}")?;
        let command = captures.name("command").unwrap().as_str().parse()?;
        let sx = captures
            .name("sx")
            .and_then(|v| v.as_str().parse().ok())
            .unwrap();
        let sy = captures
            .name("sy")
            .and_then(|v| v.as_str().parse().ok())
            .unwrap();
        let ex = captures
            .name("ex")
            .and_then(|v| v.as_str().parse().ok())
            .unwrap();
        let ey = captures
            .name("ey")
            .and_then(|v| v.as_str().parse().ok())
            .unwrap();
        let start = (sx, sy);
        let end = (ex, ey);
        Ok(Instruction {
            command,
            start,
            end,
        })
    }
}

#[aoc_generator(day6)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Instruction]) -> usize {
    input
        .iter()
        .fold(
            HashSet::new(),
            |mut acc,
             Instruction {
                 command,
                 start,
                 end,
             }| {
                let &(sx, sy) = start;
                let &(ex, ey) = end;

                for point in iproduct!((sx..=ex), (sy..=ey)) {
                    match command {
                        Command::TurnOn => {
                            acc.insert(point);
                        }
                        Command::TurnOff => {
                            acc.remove(&point);
                        }
                        Command::Toggle => {
                            if acc.contains(&point) {
                                acc.remove(&point);
                            } else {
                                acc.insert(point);
                            }
                        }
                    };
                }

                acc
            },
        )
        .len()
}

#[aoc(day6, part2)]
fn part2(input: &[Instruction]) -> usize {
    input
        .iter()
        .fold(
            HashMap::<Point, usize>::new(),
            |mut acc,
             Instruction {
                 command,
                 start,
                 end,
             }| {
                let &(sx, sy) = start;
                let &(ex, ey) = end;

                for point in iproduct!((sx..=ex), (sy..=ey)) {
                    match command {
                        Command::TurnOn => {
                            *acc.entry(point).or_default() += 1;
                        }
                        Command::TurnOff => {
                            let _ = *acc
                                .entry(point)
                                .and_modify(|n| {
                                    *n = n.checked_sub(1).unwrap_or_default();
                                })
                                .or_default();
                        }
                        Command::Toggle => {
                            *acc.entry(point).or_default() += 2;
                        }
                    };
                }

                acc
            },
        )
        .into_values()
        .sum()
}
