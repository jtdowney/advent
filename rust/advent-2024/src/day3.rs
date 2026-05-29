use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Mul(u32, u32),
    Enable,
    Disable,
}

#[aoc_generator(day3)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don\'t|do")?;
    re.captures_iter(input)
        .map(|cap| {
            let instruction = match &cap[0] {
                "do" => Instruction::Enable,
                "don't" => Instruction::Disable,
                _ => Instruction::Mul(cap[1].parse()?, cap[2].parse()?),
            };
            Ok(instruction)
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Instruction]) -> u32 {
    input
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Instruction]) -> u32 {
    let (_, sum) = input
        .iter()
        .fold((true, 0), |(enabled, acc), instruction| match instruction {
            Instruction::Mul(a, b) if enabled => (enabled, acc + a * b),
            Instruction::Enable => (true, acc),
            Instruction::Disable => (false, acc),
            Instruction::Mul(..) => (enabled, acc),
        });
    sum
}
