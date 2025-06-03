use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::intcode::{ComputerState, parse_program, run_until_output};

#[aoc_generator(day7)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    parse_program(input)
}

#[aoc(day7, part1)]
fn part1(program: &[i64]) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|phases| {
            phases.iter().fold(0, |acc, &phase| {
                let mut state = ComputerState::new(program);
                state.inputs.push_back(phase);
                state.inputs.push_back(acc);
                let (_, output) = run_until_output(state);
                output.expect("output")
            })
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(program: &[i64]) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut states = phases
                .iter()
                .map(|&phase| {
                    let mut state = ComputerState::new(program);
                    state.inputs.push_back(phase);
                    state
                })
                .collect::<Vec<ComputerState>>();

            iter::successors(Some(0), |&input| {
                phases.iter().enumerate().try_fold(input, |acc, (i, _)| {
                    states[i].inputs.push_back(acc);
                    let (new_state, output) = run_until_output(states[i].clone());
                    states[i] = new_state;
                    output
                })
            })
            .last()
            .unwrap()
        })
        .max()
        .unwrap()
}
