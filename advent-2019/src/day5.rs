use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

enum ParameterMode {
    Position,
    Immediate,
}

fn read_opcode(instruction: isize) -> isize {
    let instruction_string = instruction.to_string();
    let length = instruction_string.len();
    if length <= 2 {
        instruction
    } else {
        instruction.to_string()[length - 2..].parse().unwrap()
    }
}

fn read_mode(instruction: isize, position: usize) -> ParameterMode {
    let instruction_string = instruction.to_string();
    let length = instruction_string.len();
    let offset = 3 + position;
    if length < offset {
        ParameterMode::Position
    } else {
        let offset = length - offset;
        let mode = instruction.to_string()[offset..=offset].parse().unwrap();
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unreachable!(),
        }
    }
}

fn read_parameter(memory: &[isize], ip: usize, position: usize) -> isize {
    let instruction = memory[ip];
    let source = ip + position + 1;
    match read_mode(instruction, position) {
        ParameterMode::Position => memory[memory[source] as usize],
        ParameterMode::Immediate => memory[source],
    }
}

fn execute_intcode(memory: &[isize], input: isize) -> Vec<isize> {
    let mut memory = memory.to_vec();
    let mut outputs = Vec::new();

    let mut ip = 0;
    loop {
        let instruction = memory[ip];
        let opcode = read_opcode(instruction);
        match opcode {
            1 => {
                let left = read_parameter(&memory, ip, 0);
                let right = read_parameter(&memory, ip, 1);
                let destination = memory[ip + 3] as usize;
                memory[destination] = left + right;
                ip += 4;
            }
            2 => {
                let left = read_parameter(&memory, ip, 0);
                let right = read_parameter(&memory, ip, 1);
                let destination = memory[ip + 3] as usize;
                memory[destination] = left * right;
                ip += 4;
            }
            3 => {
                let destination = memory[ip + 1] as usize;
                memory[destination] = input;
                ip += 2;
            }
            4 => {
                let value = read_parameter(&memory, ip, 0);
                outputs.push(value);
                ip += 2;
            }
            5 => {
                let cond = read_parameter(&memory, ip, 0);
                if cond != 0 {
                    ip = read_parameter(&memory, ip, 1) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let cond = read_parameter(&memory, ip, 0);
                if cond == 0 {
                    ip = read_parameter(&memory, ip, 1) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let left = read_parameter(&memory, ip, 0);
                let right = read_parameter(&memory, ip, 1);
                let destination = memory[ip + 3] as usize;
                memory[destination] = if left < right { 1 } else { 0 };
                ip += 4;
            }
            8 => {
                let left = read_parameter(&memory, ip, 0);
                let right = read_parameter(&memory, ip, 1);
                let destination = memory[ip + 3] as usize;
                memory[destination] = if left == right { 1 } else { 0 };
                ip += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }

    outputs
}

#[aoc_generator(day5)]
fn generate(input: &str) -> Result<Vec<isize>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().context("Failed to parse number"))
        .collect()
}

#[aoc(day5, part1)]
fn part1(memory: &[isize]) -> isize {
    let outputs = execute_intcode(memory, 1);
    *outputs.last().unwrap()
}

#[aoc(day5, part2)]
fn part2(memory: &[isize]) -> isize {
    let outputs = execute_intcode(memory, 5);
    *outputs.last().unwrap()
}
