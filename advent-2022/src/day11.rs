use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u8, u64},
    combinator::{map, value},
    multi::separated_list1,
    sequence::preceded,
};

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Double,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    throw_true: usize,
    throw_false: usize,
    inspected: usize,
}

impl Monkey {
    fn inspect(&self, item: u64) -> u64 {
        match self.operation {
            Operation::Add(n) => item + n,
            Operation::Multiply(n) => item * n,
            Operation::Double => item * item,
        }
    }
}

fn parse_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    map(
        preceded(tag("  Starting items: "), separated_list1(tag(", "), u64)),
        VecDeque::from,
    )
    .parse(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let parse_add = map(preceded(tag("old + "), u64), Operation::Add);
    let parse_multiply = map(preceded(tag("old * "), u64), Operation::Multiply);
    let parse_double = value(Operation::Double, tag("old * old"));
    preceded(
        tag("  Operation: new = "),
        alt((parse_add, parse_multiply, parse_double)),
    )
    .parse(input)
}

fn parse_test(input: &str) -> IResult<&str, u64> {
    preceded(tag("  Test: divisible by "), u64).parse(input)
}

fn parse_throw<'a>(input: &'a str, result: &'static str) -> IResult<&'a str, usize> {
    let (input, _) = tag("    If ").parse(input)?;
    let (input, _) = tag(result).parse(input)?;
    map(preceded(tag(": throw to monkey "), u64), |target| {
        target as usize
    })
    .parse(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = (tag("Monkey "), u8, tag(":")).parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, items) = parse_items(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, test) = parse_test(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, throw_true) = parse_throw(input, "true")?;
    let (input, _) = line_ending.parse(input)?;
    let (input, throw_false) = parse_throw(input, "false")?;

    let monkey = Monkey {
        items,
        operation,
        test,
        throw_true,
        throw_false,
        inspected: 0,
    };

    Ok((input, monkey))
}

#[aoc_generator(day11)]
fn generator(input: &str) -> eyre::Result<Vec<Monkey>> {
    input
        .split("\n\n")
        .map(|part| match parse_monkey(part) {
            Ok((_, m)) => Ok(m),
            Err(e) => bail!("error parsing{:?}: {}", part, e),
        })
        .collect()
}

fn solve<F>(input: &[Monkey], rounds: usize, reducer: F) -> usize
where
    F: Fn(u64) -> u64,
{
    let mut monkeys = input.to_vec();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let test = monkeys[i].test;
                let throw_true = monkeys[i].throw_true;
                let throw_false = monkeys[i].throw_false;

                let level = monkeys[i].inspect(item);
                let level = reducer(level);
                if level % test == 0 {
                    monkeys[throw_true].items.push_back(level);
                } else {
                    monkeys[throw_false].items.push_back(level);
                }

                monkeys[i].inspected += 1;
            }
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc(day11, part1)]
fn part1(input: &[Monkey]) -> usize {
    solve(input, 20, |level| level / 3)
}

#[aoc(day11, part2)]
fn part2(input: &[Monkey]) -> usize {
    let product: u64 = input.iter().map(|m| m.test).product();
    solve(input, 10000, |level| level % product)
}
