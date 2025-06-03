use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{parse_program, run_with_inputs};

#[aoc_generator(day9)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    parse_program(input)
}

#[aoc(day9, part1)]
fn part1(program: &[i64]) -> i64 {
    let outputs = run_with_inputs(program, &[1]);
    *outputs.last().unwrap()
}

#[aoc(day9, part2)]
fn part2(program: &[i64]) -> i64 {
    let outputs = run_with_inputs(program, &[2]);
    *outputs.last().unwrap()
}
