use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

const SEARCH: usize = 19690720;

#[aoc_generator(day2)]
fn generate(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .split(',')
        .map(|line| line.parse::<usize>().context("parse number"))
        .collect()
}

fn execute_intcode(noun: usize, verb: usize, memory: &mut [usize]) -> usize {
    memory[1] = noun;
    memory[2] = verb;

    let mut ip = 0;
    loop {
        match memory[ip] {
            1 => {
                let value = memory[memory[ip + 1]] + memory[memory[ip + 2]];
                let address = memory[ip + 3];
                memory[address] = value;
                ip += 4;
            }
            2 => {
                let value = memory[memory[ip + 1]] * memory[memory[ip + 2]];
                let address = memory[ip + 3];
                memory[address] = value;
                ip += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }

    memory[0]
}

#[aoc(day2, part1)]
fn part1(input: &[usize]) -> usize {
    let mut program = input.to_vec();
    execute_intcode(12, 2, &mut program)
}

#[aoc(day2, part2)]
fn part2(input: &[usize]) -> Option<usize> {
    iproduct!(0..=99, 0..=99)
        .find(|&(noun, verb)| {
            let mut program = input.to_vec();
            execute_intcode(noun, verb, &mut program) == SEARCH
        })
        .map(|(noun, verb)| 100 * noun + verb)
}
