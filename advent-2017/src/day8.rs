use std::collections::HashMap;

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, IResult, Parser};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Increment(i32),
    Decrement(i32),
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    GreaterThan(i32),
    LessThan(i32),
    GreaterThanOrEqual(i32),
    LessThanOrEqual(i32),
    Equal(i32),
    NotEqual(i32),
}

#[derive(Debug, Clone)]
struct Instruction {
    operation_register: String,
    operation: Operation,
    condition_register: String,
    condition: Condition,
}

fn register(input: &str) -> IResult<&str, String> {
    use nom::{character::complete::alpha1, combinator::map};
    map(alpha1, String::from).parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{i32, space1},
        combinator::map,
        sequence::preceded,
    };

    let operation = alt((
        map(preceded(tag("inc "), i32), Operation::Increment),
        map(preceded(tag("dec "), i32), Operation::Decrement),
    ));
    let condition = alt((
        map(preceded(tag("> "), i32), Condition::GreaterThan),
        map(preceded(tag("< "), i32), Condition::LessThan),
        map(preceded(tag(">= "), i32), Condition::GreaterThanOrEqual),
        map(preceded(tag("<= "), i32), Condition::LessThanOrEqual),
        map(preceded(tag("== "), i32), Condition::Equal),
        map(preceded(tag("!= "), i32), Condition::NotEqual),
    ));

    map(
        (
            register,
            space1,
            operation,
            space1,
            tag("if "),
            register,
            space1,
            condition,
        ),
        |(operation_register, _, operation, _, _, condition_register, _, condition)| Instruction {
            operation_register,
            operation,
            condition_register,
            condition,
        },
    ).parse(input)
}

#[aoc_generator(day8)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            instruction(line)
                .finish()
                .map(|(_, i)| i)
                .map_err(|e| anyhow!("Invalid instruction {:?}: {}", line, e))
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let mut pc = 0;
    let mut state = HashMap::new();

    while let Some(instruction) = input.get(pc) {
        let condition_register = state
            .entry(instruction.condition_register.clone())
            .or_default();
        let condition = match instruction.condition {
            Condition::GreaterThan(v) => *condition_register > v,
            Condition::LessThan(v) => *condition_register < v,
            Condition::GreaterThanOrEqual(v) => *condition_register >= v,
            Condition::LessThanOrEqual(v) => *condition_register <= v,
            Condition::Equal(v) => *condition_register == v,
            Condition::NotEqual(v) => *condition_register != v,
        };

        let operation_register = state
            .entry(instruction.operation_register.clone())
            .or_default();
        if condition {
            match instruction.operation {
                Operation::Increment(v) => *operation_register += v,
                Operation::Decrement(v) => *operation_register -= v,
            }
        }

        pc += 1;
    }

    state.into_values().max().unwrap_or_default()
}

#[aoc(day8, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let mut pc = 0;
    let mut state = HashMap::new();
    let mut max = 0;

    while let Some(instruction) = input.get(pc) {
        let condition_register = state
            .entry(instruction.condition_register.clone())
            .or_default();
        let condition = match instruction.condition {
            Condition::GreaterThan(v) => *condition_register > v,
            Condition::LessThan(v) => *condition_register < v,
            Condition::GreaterThanOrEqual(v) => *condition_register >= v,
            Condition::LessThanOrEqual(v) => *condition_register <= v,
            Condition::Equal(v) => *condition_register == v,
            Condition::NotEqual(v) => *condition_register != v,
        };

        let operation_register = state
            .entry(instruction.operation_register.clone())
            .or_default();
        if condition {
            match instruction.operation {
                Operation::Increment(v) => *operation_register += v,
                Operation::Decrement(v) => *operation_register -= v,
            }

            max = max.max(*operation_register);
        }

        pc += 1;
    }

    max
}
