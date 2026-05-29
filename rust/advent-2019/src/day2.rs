use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

use crate::intcode::{ComputerState, parse_program, run_to_completion};

const SEARCH: i64 = 19690720;

#[aoc_generator(day2)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    parse_program(input)
}

fn execute_intcode(noun: i64, verb: i64, program: &[i64]) -> i64 {
    let mut state = ComputerState::new(program);
    state.memory[1] = noun;
    state.memory[2] = verb;

    let (final_state, _) = run_to_completion(state);
    final_state.memory[0]
}

#[aoc(day2, part1)]
fn part1(program: &[i64]) -> i64 {
    execute_intcode(12, 2, program)
}

#[aoc(day2, part2)]
fn part2(program: &[i64]) -> Option<i64> {
    iproduct!(0..=99, 0..=99)
        .find(|&(noun, verb)| execute_intcode(noun, verb, program) == SEARCH)
        .map(|(noun, verb)| 100 * noun + verb)
}
