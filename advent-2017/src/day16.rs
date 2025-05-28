use std::str::FromStr;

use anyhow::{Context, bail};
use aoc_runner_derive::{aoc, aoc_generator};

const START_PROGRAM: &[char; 16] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
];

#[derive(Clone, Copy)]
enum Operation {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = &s[0..1];
        let operand = &s[1..];
        match prefix {
            "s" => Ok(Operation::Spin(operand.parse()?)),
            "x" => {
                let mut parts = operand.split('/');
                let a = parts.next().context("unable to find part")?.parse()?;
                let b = parts.next().context("unable to find part")?.parse()?;
                Ok(Operation::Exchange(a, b))
            }
            "p" => {
                let mut parts = operand.split('/');
                let a = parts
                    .next()
                    .and_then(|p| p.chars().next())
                    .context("unable to find part")?;
                let b = parts
                    .next()
                    .and_then(|p| p.chars().next())
                    .context("unable to find part")?;
                Ok(Operation::Partner(a, b))
            }
            _ => bail!("unknown operation: {}", prefix),
        }
    }
}

impl Operation {
    fn transform(&self, mut result: Vec<char>) -> Vec<char> {
        match *self {
            Operation::Spin(size) => result.rotate_right(size),
            Operation::Exchange(a, b) => result.swap(a, b),
            Operation::Partner(a, b) => {
                let apos = result.iter().position(|&c| c == a).unwrap();
                let bpos = result.iter().position(|&c| c == b).unwrap();
                result.swap(apos, bpos);
            }
        }

        result
    }
}

#[aoc_generator(day16)]
fn generator(input: &str) -> anyhow::Result<Vec<Operation>> {
    input.split(',').map(|part| part.parse()).collect()
}

#[aoc(day16, part1)]
fn part1(input: &[Operation]) -> String {
    let start = START_PROGRAM.to_vec();
    input
        .iter()
        .fold(start, |acc, op| op.transform(acc))
        .iter()
        .collect()
}

#[aoc(day16, part2)]
fn part2(input: &[Operation]) -> String {
    let mut program = START_PROGRAM.to_vec();
    let mut i = 0;
    let iterations = 1_000_000_000;
    while i < iterations {
        program = input
            .iter()
            .fold(program, |acc, op| op.transform(acc))
            .into_iter()
            .collect();

        if program == START_PROGRAM {
            i += ((iterations / (i + 1)) - 1) * (i + 1);
        }

        i += 1;
    }

    program.iter().collect()
}
