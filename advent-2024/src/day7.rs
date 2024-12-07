use std::str::FromStr;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Equation {
    target: u64,
    remaining: Vec<u64>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, remaining) = s.split_once(": ").context("Invalid input")?;
        let target = target.parse()?;
        let mut remaining = remaining
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        remaining.reverse();
        Ok(Self { target, remaining })
    }
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<Vec<Equation>> {
    input.lines().map(str::parse).collect()
}

fn is_valid(eq: &Equation, concat: bool) -> bool {
    if eq.remaining.len() == 1 {
        return eq.remaining[0] == eq.target;
    }

    let mut remaining = eq.remaining.clone();
    let left = remaining.pop().unwrap();
    if left > eq.target {
        return false;
    }

    let right = remaining.pop().unwrap();

    let mut a_remaining = remaining.clone();
    a_remaining.push(left + right);

    let mut b_remaining = remaining.clone();
    b_remaining.push(left * right);

    let a = Equation {
        remaining: a_remaining,
        ..*eq
    };
    let b = Equation {
        remaining: b_remaining,
        ..*eq
    };

    let result = is_valid(&a, concat) || is_valid(&b, concat);
    if concat {
        let mut c_remaining = remaining;
        let concated = format!("{}{}", left, right).parse().unwrap();
        c_remaining.push(concated);

        let c = Equation {
            remaining: c_remaining,
            ..*eq
        };

        result || is_valid(&c, concat)
    } else {
        result
    }
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| is_valid(eq, false))
        .map(|eq| eq.target)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| is_valid(eq, true))
        .map(|eq| eq.target)
        .sum()
}
