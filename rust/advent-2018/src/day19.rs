use aoc_runner_derive::{aoc, aoc_generator};

use crate::vm::{Machine, Program};

#[aoc_generator(day19)]
fn generator(input: &str) -> anyhow::Result<Program> {
    Program::parse(input)
}

#[aoc(day19, part1)]
fn part1(input: &Program) -> usize {
    let mut machine = Machine::new(input.ip_register);
    while machine.step_alt(&input.instructions) {}
    machine.registers[0]
}

#[aoc(day19, part2)]
fn part2(input: &Program) -> usize {
    let mut machine = Machine::new(input.ip_register);
    machine.registers[0] = 1;

    let mut count = 0;
    while machine.step_alt(&input.instructions) {
        count += 1;
        if count > 25 {
            break;
        }
    }

    let n = machine.registers[4];
    n + (1..=n / 2).filter(|x| n.is_multiple_of(*x)).sum::<usize>()
}
